# craf
A CLI tool to shorthand long shell commands.

> [!Note]
> This project is very experimental.
> Please be aware this repository may be archived.

## Usage
1. Register the shell command with `craf --create`. `craf` will save it under `~/.craf`.
2. To call it, run `craf <name>`

### Help Message
```console
$ craf --help
A CLI tool to shorthand long shell commands.

Usage: craf [OPTIONS] [NAME]

Arguments:
  [NAME]

Options:
      --list      List tasks
      --create    Create new task
      --delete    Delete a task
      --describe  Describe a task
  -h, --help      Print help
  -V, --version   Print version
```

### Example UseCase
```console
$ craf --create
> name: dock
> shell command (like `echo a`): docker ps -a
> description: show docker containers

$ craf --list
1 command(s) found.

Commands:
  dock

$ craf dock
Run following command..
  docker ps -a

CONTAINER ID   IMAGE                  COMMAND                   CREATED       STATUS                     PORTS     NAMES
askad0   tinygo/tinygo:latest   "tinygo build -o mai…"   3 weeks ago   Exited (0) 3 weeks ago               app-sldsl

Command completed with status 0
```
