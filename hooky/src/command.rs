use serde::Deserialize;
use utils::Error;

use crate::config::{CommandConfig, OutputMode, ShellMode};
use std::fmt::Display;
use std::process::{Child, Command as Process, Stdio};
use std::str::FromStr;
use tokio::runtime::Runtime;
use tokio::task::spawn_blocking;

#[derive(PartialEq, PartialOrd, Debug, Deserialize)]
#[allow(dead_code)]
pub enum Status {
    /// si el comando todavia no se ejecuto
    Pending,
    /// si el comando NO termino
    Running,
    /// si el comando termino satisfactoriamente o no, pero que termino
    Finished(i32),
    /// si hubo algun error al momento de iniciar el comando
    Failed(String),
}

#[derive(Debug, PartialEq, Eq)]
pub enum CommandErrorKind {
    InvalidParse,
    ExecutionFinalizated,
    ExecutionError,
}

#[derive(Debug, Error)]
pub struct CommandError {
    // tipo de error
    pub kind: CommandErrorKind,
    /// mensaje que va a dejar el error
    pub msg: String,
}

#[allow(dead_code)]
/// En la estructura del comando vamos a guardar, el nombre, sus argumentos y su estado
#[derive(Debug)]
pub struct Command {
    name: String,
    args: Vec<String>,
    state: Status,
    child: Option<Child>,
    config: CommandConfig,
}

#[allow(dead_code)]
impl Command {
    pub fn new(config: CommandConfig, code: String) -> Self {
        let mut command = code.split(' ').map(|f| f.to_string()).collect::<Vec<_>>();
        let name = command[0].to_owned();
        let args = command.drain(1..).collect::<Vec<String>>();
        Self {
            name,
            args,
            state: Status::Pending,
            child: None,
            config,
        }
    }

    // propiedades
    /// retorna el nombre del comando
    pub fn name(&self) -> String {
        self.name.to_owned()
    }

    /// retorna los argumentos del comando
    pub fn args(&self) -> Vec<String> {
        self.args.to_owned()
    }

    /// comprueba si el comando esta siendo ejecutado
    pub fn is_running(&self) -> bool {
        self.state == Status::Running
    }

    /// comprueba si el comando ya finalizo
    pub fn is_finished(&self) -> bool {
        matches!(self.state, Status::Finished(_))
    }

    /// retorna el codigo de salida en caso de que haya terminado, sino retorna `None`
    pub fn exit_code(&self) -> Option<i32> {
        match self.state {
            Status::Finished(code) => Some(code),
            _ => None,
        }
    }

    /// retorna el estado
    pub fn state(&self) -> &Status {
        &self.state
    }

    /// setear la configuracion
    pub fn set_config(&mut self, config: CommandConfig) {
        self.config = config
    }

    /// ejecuta el `child`
    pub fn execute(&mut self) {
        if self.is_running() {
            return;
        }
        let (c_name, c_arg) = match self.config.shell_mode() {
            ShellMode::Cmd => ("cmd", "/C"),
            ShellMode::Powershell => ("powershell", "-c"),
            ShellMode::Shell => ("sh", "-c"),
        };
        let mut binding = Process::new(c_name);
        let child = binding.arg(c_arg).args(vec![format!(
            "{name} {args}",
            name = self.name,
            args = self.args.join(" ")
        )]);
        if self.config.output_mode() == OutputMode::Supress {
            child.stdout(Stdio::null());
            child.stderr(Stdio::null());
        }
        match child.spawn() {
            Ok(e) => {
                self.state = Status::Running;
                self.child = Some(e);
            }
            Err(err) => self.state = Status::Failed(err.kind().to_string()),
        };
    }

    /// espera a que el proceso se ejecute de forma asincrona y modifica el `state` del comando
    pub async fn wait_async(&mut self) {
        if self.is_finished() || self.state == Status::Pending {
            return;
        };
        if let Some(mut child) = self.child.take() {
            self.state = spawn_blocking(move || match child.wait() {
                Ok(code) => Status::Finished(code.code().unwrap_or(1)),
                Err(_) => Status::Finished(1),
            })
            .await
            .unwrap();
        }
    }

    /// espera a que el proceso se ejecute de forma sincrona y modifica el `state` del comando
    pub fn wait_sync(&mut self) {
        if self.is_finished() || self.state == Status::Pending {
            return;
        };
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            if let Some(mut child) = self.child.take() {
                let exit_status = child.wait();
                let code = match exit_status {
                    Ok(code) => {
                        let c = code;
                        c.code().unwrap_or(1)
                    }
                    Err(_) => 1,
                };
                self.state = Status::Finished(code);
            }
        });
    }

    /// permite matar el proceso
    pub fn kill(&mut self) -> Result<(), CommandError> {
        if self.child.is_none() || matches!(self.state, Status::Pending | Status::Finished(_)) {
            return Err(CommandError {
                kind: CommandErrorKind::ExecutionFinalizated,
                msg: "command killed yet".into(),
            });
        };
        match self.child.as_mut().unwrap().kill() {
            Ok(_) => {
                self.state = Status::Finished(0);
                Ok(())
            }
            Err(err) => {
                self.state = Status::Finished(1);
                Err(CommandError {
                    kind: CommandErrorKind::ExecutionError,
                    msg: err.kind().to_string(),
                })
            }
        }
    }
}

impl FromStr for Command {
    type Err = CommandErrorKind;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(CommandErrorKind::InvalidParse);
        }
        let mut args = s.split_whitespace();
        let name = args
            .next()
            .ok_or(CommandErrorKind::InvalidParse)?
            .to_string();
        let arguments: Vec<String> = args.map(|x| x.to_string()).collect();
        Ok(Self {
            name,
            args: arguments,
            state: Status::Pending,
            child: None,
            config: CommandConfig::default(),
        })
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.name, self.args.join(" "))
    }
}
