# designdoc
Command Shortener.

## Commands
```bash
craf command list
craf command create # this turns up interactive prompt
craf command describe --name <name>
craf command update-somthing-setting --name <name> --value <value>
craf command remove --name <name>
craf run <name> # also, original command arguments can be passed here.
```

## ~/.craftant/commands.json
```json
{
    "commands": [
        {
            "name": "aaa",
            "description": "Commands for test.",
            "bin": "echo",
            "args": [
                "aa",
                "--hello",
                "aaa"
            ]
        }
    ]
}
```

## TODO
### refactor command name
resgiter や unregister が一般的なコマンド名なのかわからない. そのため調査する
### comamds という名前
craftant の subcommand と混同する. より解像度が高い名前があれば使いたい

