# designdoc
Dynamically configurable command.  

## About
`craftant` is a CLI command and its subcommands are dynamically configurable.  
By running `craftant command add`, you can add a subcommand and register a shell command to run.  
Registered shell commands will be invoked when craftant subcommands are called.  

## Features
- Dynamically add subcommands.
- Configurations are stored in `~/.craftant`.

## Usage
```bash
$ craftant command add
Please enter command name to register: echo
Which command would you like to run?: echo something
Please enter description: This command shows something.
$ craftant command list
Commands:
   craftant echo    This command shows something.
$ craftant echo
something
```

## Commands
```bash
craftant command list
craftant command add  --name <name> # this turns up interactive prompt
craftant command describe --name <name>
craftant command update-somthing-setting --name <name> --value <value>
craftant command remove --name <name>
craftant run <name> # also, original command arguments can be passed here.
```

## ~/.craftant/commands/{name}.json
```json
{
    "description": "Commands for test.",
    "bin": "echo",
    "args": [
        "aa",
        "--hello",
        "aaa"
    ]
}
```

## イメージ
```bash
craftant command update-arguments --command-name <command-name> --name <name> --value <value>
```
こういうのもできる。
あんまり機能をつけても、実行するコマンドを想像できなくなるだけなので、できればシンプルにしたいなあ。せいぜいマッピング程度。

## Development Plan
### v0.1.0
とりあえず動くようにする
### v0.2.0
コマンド体系見直し
