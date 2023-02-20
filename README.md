# fws-rs
<p float=left>
    <img src= "https://img.shields.io/static/v1?style=flat&message=Windows&color=0078D6&logo=Windows&logoColor=FFFFFF&label="/>
    <img src= "https://img.shields.io/static/v1?style=flat&message=Linux&color=222222&logo=Linux&logoColor=FCC624&label="/>
    <img src= "https://img.shields.io/badge/License-MIT-green.svg"/>
    <img src= "https://img.shields.io/github/v/release/ChirujanoCodding/fws-rs?color=lightgray"/>
</p>

fws-rs is an application written in rust that acts as a configurable file watcher, capable of detecting real-time changes and running custom commands. Configuration can be done via a JSON file or as arguments during runtime.
## ⭐ Features

- Watch for file system changes (create, modify, delete)
- Execute custom commands
- Configuration via JSON file or command line arguments
- Recursive directory watching
- Observe custom files with patterns extensions
## 🚀 Getting started
### 📥 Installation

Clone the repo or [download](https://github.com/ChirujanoCodding/fws-rs/releases/tag) the most recently release from github.

```bash
git clone https://github.com/ChirujanoCodding/fws-rs.git
```

### 👷‍♂️ Building the project

In the folder `fws-rs` run:

```bash
cargo build --release
```
## 📚 Usage & Examples

Once you have the program builded or downloaded from the most recently release, insert the `fws-rs.exe` or `fws-rs` (depends your OS) in your project.

1. Using a configuration file:

```bash
fws-rs --config "config.json"
```

(You must need to create a `config.json` file like this)
```json
{
    "watch": "your/path", // the path you want to observe.
    "exec": "python main.py", // the command you want to be execute when apply changes on files.
    "recursive": true | false // if you want to look specific path or recursive (default False).
    "on_events_only": true, // if you want to execute commands only if ocurrs an event (default False).
    "patterns: [".rs", ".py", ".js"] // your .extensions (default []).
}
```

2. Using configuration arguments:

```bash
fws-rs --watch "your/path" --exec "your script"
```


## 📷 Screenshots

Executing with different configs

![image](https://user-images.githubusercontent.com/84428770/220008335-7b6bb319-0bf2-45ac-b3da-463a94b082f4.png)

![image](https://user-images.githubusercontent.com/84428770/220008772-ecf6a201-dbd1-4088-80d7-46a6a7b74e22.png)


Different logs:

![image](https://user-images.githubusercontent.com/84428770/220009127-f6cf611d-3e32-48af-bf1a-ea9ae7225bdc.png)


## 💼 License

This project is under the MIT License.


