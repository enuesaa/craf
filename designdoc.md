# designdoc
Command alias command.

## About
`craftant` is a command that has dynamically configurable subcommands.  
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
craftant command add --name <name> --command <command> --description <description>
craftant command add # this turns up interactive prompt
craftant command describe --name <name>
craftant command update-somthing-setting --name <name> --value <value>
craftant command remove --name <name>
craftant run <name> # also, original command arguments can be passed here.
```

## ~/.craftant/commands/{name}.json
```json
{
    "name": "ll",
    "description": "Commands for test.",
    "command": "ls -la"
}
```

## オプションで渡した値が環境変数に入る
```bash
craftant ll --aa bb
```
がシェル変数 AA=bb をセットしコマンドを呼ぶ
```bash
ls -la $AA
```

## Development Plan
### v0.1.0
とりあえず動くようにする
### v0.2.0
コマンド体系見直し
