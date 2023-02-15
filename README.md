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

2. Using configuration arguments:

```bash
fws-rs --watch "your/path" --exec "your script"
```


## 📷 Screenshots

Executing with different configs

![image](https://user-images.githubusercontent.com/84428770/218912861-7281f391-527e-42d0-acd4-350c7541507a.png)

![image](https://user-images.githubusercontent.com/84428770/218913004-f30a16af-f826-4a47-9676-769c8d2761c5.png)


Different logs:

![image](https://user-images.githubusercontent.com/84428770/218913462-b55ef98b-d2b0-48ba-a1d1-adcfcad6876d.png)


## 💼 License

This project is under the MIT License.


