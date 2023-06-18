# designdoc
## About
By running `crafant command add`, you can register a shell command.  
By running `crafant <name>`, you can invoke registered shell commands.

## Usage
```bash
$ crafant command add
name: echo
shell command (like `echo a`): echo something
description: This command shows something.

$ crafant command list
1 command(s) found.

Commands:
   crafant run echo    This command shows something.

$ crafant run echo
something
```

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
### v0.1.0
とりあえず動くようにする
### v0.2.0
コマンド体系見直し. Builderパターンにする
```
crafant command add
crafant <name>
```
