# designdoc
Dynamically configurable command.  

You can add subcommands or options of `craftant` command dynamically.
And you can also register a shell command to run when subcommands are invoked.
These configurations are stored in `~/.craftant`.

## Usage
```bash
$ craftant command add
Please input command name to register: echo
Which command would you like to run?: echo something
Please input command description: This command shows something.
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
