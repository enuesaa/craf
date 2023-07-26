# crafant
A CLI tool to shorthand long shell commands.

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
> name: dock
> shell command (like `echo a`): docker ps -a
> description: show docker containers

$ crafant list
1 command(s) found.

Commands:
  dock
$ crafant run dock
Run following command..
  docker ps -a

CONTAINER ID   IMAGE                  COMMAND                   CREATED       STATUS                     PORTS     NAMES
askad0   tinygo/tinygo:latest   "tinygo build -o mai…"   3 weeks ago   Exited (0) 3 weeks ago               app-sldsl

Command completed with status 0
```
