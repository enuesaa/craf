**Work in progress...**
# crafant
A CLI tool to memorize long shell commands.

## Commands
```bash
crafant list
crafant describe <name>
crafant add 
crafant remove <name>
crafant run <name>
```

## Usage
```bash
$ crafant add
name: echo
shell command (like `echo a`): echo something
description: This command shows something.

$ crafant list
1 command(s) found.

Commands:
   crafant run echo    This command shows something.

$ crafant run echo
something
```
