# designdoc
## About
By running `crafant command add`, you can register a shell command.  
By running `crafant <name>`, you can invoke registered shell commands.

## Commands
```bash
crafant command list
crafant command add --name <name> --command <command> --description <description>
crafant command add # this turns up interactive prompt
crafant command describe --name <name>
crafant command update-somthing-setting --name <name> --value <value>
crafant command remove --name <name>
crafant run <name> # also, original command arguments can be passed here.
```

## ~/.crafant/commands/{name}.json
```json
{
    "name": "ll",
    "description": "Commands for test.",
    "command": "ls -la"
}
```

## オプションで渡した値が環境変数に入る
```bash
crafant ll --aa bb
```
がシェル変数 AA=bb をセットしコマンドを呼ぶ
```bash
ls -la $AA
```

## Development Plan
### v0.0.1
とりあえず動くようにする
### v0.1.0
設定ファイルの形式見直し。大してユースケースがない割にファイルが無数にあってもしょうがない
### v0.2.0
コマンド体系見直し. Builderパターンにする
```
crafant command add
crafant <name>
```
