# Bookmark &emsp; ![Rust](https://github.com/tiohsa/bm-bookmark/workflows/Rust/badge.svg)

Bookmark the directory path.

## install

```bash
$ cargo install --path .
```

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
    remove    remove bookmark
```

## Bookmark file

`$HOME/.cache/bm-bookmark`

## for fish shell

```bash
function bm
  set length (count $argv)
  if test $length -eq 0
    bookmark
  else if test $length -eq 1; and  test $arvg[1] != "-h" or test $argv[1] != "help"
    cd (bookmark $argv)
  else
    bookmark $argv
  end
end
```
