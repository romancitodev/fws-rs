// Importa el código que se va a probar
use hooky::command;
use hooky::config;

#[cfg(test)]
mod tests {

    // Importa las herramientas de testing de Rust
    use super::command::*;
    use super::config::*;
    // Prueba la función `new()`
    #[tokio::test]
    async fn test_async_new() {
        let config = CommandConfig::new(OutputMode::Supress, ShellMode::Cmd);
        let mut cmd = Command::new(config, r#"echo "im a single command""#.into());
        assert_eq!(cmd.name(), "echo");
        assert_eq!(cmd.args(), vec!["\"im", "a", "single", "command\""]);
        cmd.execute();
        cmd.wait_async().await;
        assert_eq!(cmd.exit_code(), Some(0));
    }

    #[tokio::test]
    async fn test_async_new_parsed() {
        let config = CommandConfig::new(OutputMode::Supress, ShellMode::Cmd);
        let mut cmd = "echo \"im a parsed command\"".parse::<Command>().unwrap();
        cmd.set_config(config);
        assert_eq!(cmd.name(), "echo");
        assert_eq!(cmd.args(), vec!["\"im", "a", "parsed", "command\""]);
        cmd.execute();
        cmd.wait_async().await;
        assert_eq!(cmd.exit_code(), Some(0));
    }

    #[test]
    fn test_error_parsed() {
        let cmd = "".parse::<Command>().unwrap_err();
        assert_eq!(cmd, CommandErrorKind::InvalidParse);
    }

    // Prueba la función `execute()`
    #[tokio::test]
    async fn test_async_execute() {
        let config = CommandConfig::default();
        let mut cmd = Command::new(config, "python tests/functions/main.py".into());
        cmd.execute();
        assert!(cmd.is_running());
        cmd.wait_async().await;
        assert!(cmd.is_finished());
        assert_eq!(cmd.exit_code(), Some(0));
    }

    #[test]
    fn test_premature_killing() {
        let config = CommandConfig::default();
        let mut cmd = Command::new(config, "python tests/functions/main.py".into());
        cmd.execute();
        assert!(cmd.is_running());
        cmd.kill().unwrap();
        assert!(cmd.is_finished());
        assert_eq!(cmd.exit_code(), Some(0));
    }

    /// Prueba la funcion `execute() + wait()` pero este retornara error.
    #[tokio::test]
    async fn test_async_should_bad_exit() {
        let config = CommandConfig::new(OutputMode::Supress, ShellMode::Powershell);
        let mut cmd = Command::new(config, "python tests/functions/bad.py".into());
        cmd.execute();
        assert!(cmd.is_running());
        cmd.wait_async().await;
        assert!(cmd.is_finished());
        assert_eq!(cmd.exit_code(), Some(1));
    }

    /// Prueba la funcion `execute() + wait()` pero este retornara error.
    #[test]
    fn test_sync_should_bad_exit() {
        let config = CommandConfig::new(OutputMode::Supress, ShellMode::Powershell);
        let mut cmd = Command::new(config, "python tests/functions/bad.py".into());
        cmd.execute();
        assert!(cmd.is_running());
        cmd.wait_sync();
        assert!(cmd.is_finished());
        assert_eq!(cmd.exit_code(), Some(1));
    }

    // Prueba el estado pendiente
    #[tokio::test]
    async fn test_async_ideal_command() {
        let config = CommandConfig::default();
        let mut cmd = Command::new(config, "python tests/functions/main.py".into());
        assert_eq!(cmd.state(), &Status::Pending);
        cmd.execute();
        assert_eq!(cmd.state(), &Status::Running);
        assert!(cmd.is_running());
        cmd.wait_async().await;
        assert!(cmd.is_finished());
        assert_eq!(cmd.state(), &Status::Finished(0));
        assert_eq!(cmd.exit_code(), Some(0));
    }

    #[tokio::test]
    async fn test_async_input() {
        let config = CommandConfig::new(OutputMode::Supress, ShellMode::Powershell);
        let mut cmd = Command::new(config, "python tests/functions/input.py".into());
        assert_eq!(cmd.state(), &Status::Pending);
        cmd.execute();
        assert_eq!(cmd.state(), &Status::Running);
        assert!(cmd.is_running());
        cmd.wait_async().await;
        assert!(cmd.is_finished());
        assert_eq!(cmd.state(), &Status::Finished(0));
        assert_eq!(cmd.exit_code(), Some(0));
    }

    // Prueba la función `execute()` con multiples comandos
    #[tokio::test]
    async fn test_async_multiple_commands() {
        let config = CommandConfig::new(OutputMode::Supress, ShellMode::Cmd);
        let mut commands = "echo \"im a multiple command\" && python tests/functions/main.py"
            .split(" && ")
            .map(|command| Command::new(config, command.into()))
            .collect::<Vec<_>>();
        for c in commands.iter_mut() {
            assert_eq!(c.state(), &Status::Pending);
            c.execute();
            assert!(matches!(c.state(), &Status::Running | &Status::Finished(_)));
            c.wait_async().await;
            assert!(c.is_finished());
            assert_eq!(c.state(), &Status::Finished(0));
            assert_eq!(c.exit_code(), Some(0));
        }
    }

    #[tokio::test]
    async fn test_async_timeout_cmd() {
        let config = CommandConfig::new(OutputMode::Supress, ShellMode::Cmd);
        let mut cmd = Command::new(config, "timeout 5".into());
        cmd.execute();
        assert!(cmd.is_running());
        cmd.wait_async().await;
        assert!(cmd.is_finished());
        assert_eq!(cmd.exit_code(), Some(0));
    }

    #[tokio::test]
    async fn test_async_timeout_powershell() {
        let config = CommandConfig::new(OutputMode::Supress, ShellMode::Powershell);
        let mut cmd = Command::new(config, "sleep 5".into());
        cmd.execute();
        assert!(cmd.is_running());
        cmd.wait_async().await;
        assert!(cmd.is_finished());
        assert_eq!(cmd.exit_code(), Some(0));
    }

    #[test]
    fn test_sync_timeout_cmd() {
        let config = CommandConfig::new(OutputMode::Supress, ShellMode::Cmd);
        let mut cmd = Command::new(config, "timeout 5".into());
        cmd.execute();
        assert_eq!(cmd.state(), &Status::Running);
        cmd.wait_sync();
        assert_eq!(cmd.state(), &Status::Finished(0));
        assert_eq!(cmd.exit_code(), Some(0));
    }
    #[test]
    fn test_sync_timeout_powershell() {
        let config = CommandConfig::new(OutputMode::Supress, ShellMode::Powershell);
        let mut cmd = Command::new(config, "sleep 5".into());
        cmd.execute();
        assert_eq!(cmd.state(), &Status::Running);
        cmd.wait_sync();
        assert_eq!(cmd.state(), &Status::Finished(0));
        assert_eq!(cmd.exit_code(), Some(0));
    }

    #[test]
    fn test_sync_execute() {
        let config = CommandConfig::new(OutputMode::Supress, ShellMode::Cmd);
        let mut cmd = Command::new(config, "echo im a single command".into());
        cmd.execute();
        cmd.wait_sync();
        assert!(cmd.is_finished());
    }

    #[test]
    fn test_sync_input() {
        let config = CommandConfig::new(OutputMode::Supress, ShellMode::Cmd);
        let mut cmd = Command::new(config, "python tests/functions/input.py".into());
        cmd.execute();
        assert_eq!(cmd.state(), &Status::Running);
        cmd.wait_sync();
        assert_eq!(cmd.state(), &Status::Finished(0));
        assert_eq!(cmd.exit_code(), Some(0));
    }

    #[tokio::test]
    async fn test_kill_success() {
        let config = CommandConfig::new(OutputMode::Supress, ShellMode::Cmd);
        let mut cmd = Command::new(config, "timeout 10".into());
        cmd.execute();
        let res = cmd.kill();
        assert!(res.is_ok());
        assert!(matches!(cmd.state(), Status::Finished(_)));
    }

    #[tokio::test]
    async fn test_kill_already_finished() {
        let config = CommandConfig::new(OutputMode::Allow, ShellMode::Cmd);
        let mut cmd = Command::new(config, "echo hello world!".into());
        cmd.execute();
        cmd.wait_async().await;
        let res = cmd.kill();
        assert!(res.is_err());
        assert_eq!(
            res.unwrap_err().kind,
            CommandErrorKind::ExecutionFinalizated
        );
    }

    #[tokio::test]
    async fn test_command_multiple_line_async() {
        let config = CommandConfig::new(OutputMode::Allow, ShellMode::Cmd);
        let mut cmd = Command::new(config, "echo hello world && echo good bye world".into());
        cmd.execute();
        cmd.wait_async().await;
        assert_eq!(cmd.state(), &Status::Finished(0))
    }

    #[test]
    fn test_command_multiple_line_sync() {
        let config = CommandConfig::new(OutputMode::Allow, ShellMode::Cmd);
        let mut cmd = Command::new(config, "echo hello world && echo good bye world".into());
        cmd.execute();
        cmd.wait_sync();
        assert_eq!(cmd.state(), &Status::Finished(0))
    }
}
