# designdoc
## Commands
```bash
craftant list
craftant register # this turns up interactive prompt
craftant info <name>
craftant unregister <name>
craftant run <name>
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

