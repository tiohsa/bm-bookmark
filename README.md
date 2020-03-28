# Bookmark

Bookmark the directory path.

## Usage

```bash
USAGE:
    bookmark [NAME] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <NAME>    get bookmark by name.

SUBCOMMANDS:
    add       add directory to bookmark
    help      Prints this message or the help of the given subcommand(s)
    list      show bookmark list
    remove    remove bookmark
```

## Bookmark file

`$HOME/.cache/bm-bookmark`

## for fish shell

```bash
function bm-bookmark
  switch $argv[1]
    case list
      bookmark $argv
    case add
      bookmark $argv
    case remove
      bookmark $argv
    case '*'
      cd (bookmark $argv)
  end
end

alias bm bm-bookmark
```
