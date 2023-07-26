# planning
## Commands
```bash
crafant update-somthing-setting --name <name> --value <value>
```

## オプションで渡した値が環境変数に入る
```bash
crafant ll --aa bb
```
がシェル変数 AA=bb をセットしコマンドを呼ぶ
```bash
ls -la $AA
```
