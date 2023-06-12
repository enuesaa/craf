# designdoc
Command Shortener.

## Commands
```bash
craftant command list
craftant command create # this turns up interactive prompt
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
