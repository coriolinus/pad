# `pad`: command line text padding

Sometimes you have a command line sequence from which you need to `cut` some columns. Sometimes getting that command to work right requires that you previously `tr -s ' '`. Sometimes you have this solution all coded up before discovering the `column -t` command.

In that case, you need `pad`.

## Usage

```text
$ pad -h
pad 0.1.0
Peter Goodspeed-Niklaus <peter.r.goodspeedniklaus@gmail.com>


USAGE:
    pad [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --align <align>...     Specify a column's alignment. May be repeated.
    -d, --delimiter <delim>    Specify the delimiter with which to distinguish input fields
    -f, --file <file>          Read from the named file instead of stdin
    -s, --separator <sep>      Specify the separator with which to separate output fields
```

If an input file is specified, that file is read. Otherwise, `pad` reads from standard input.

The input is split into columns according to the delimiter, and space-padded such that every column has consistent width. It is then output with the separator separating the columns.

Columns are aligned left by default, though column alignment may be set per column using the `-a` option:

- "l", "L", and "<" set left alignment
- "r", "R", and ">" set right alignment
- "c", "C", and "^" set center alignment

The `-a` option must be specified once per column, and sets columns starting from the leftmost.

## Examples

### Basic usage

```sh
$ ls -l | tr -s ' ' | cut -d' ' -f5,9 | pad

11469 Cargo.lock
216   Cargo.toml
0     README.md
512   src
512   target
```

### Set column alignment

```sh
$ ls -l | tr -s ' ' | cut -d' ' -f5,9 | pad -ar -ac

11469 Cargo.lock
  216 Cargo.toml
    0 README.md
  512    src
  512   target
```

### Split and separate by a custom character

```sh
$ cat Cargo.toml | grep -Po "\d+\.\d+\.\d+" | pad -d. -s. -ar -ar -ar
0. 1.0
2.30.0
0. 1.1
0. 7.6
1. 2.0
```

### Read from a file instead of stdin

```sh
$ pad -f .gitignore -d'/'

           target
**         *.rs.bk
Cargo.lock
```

### Split/separate on an equals sign:

```sh
$ pad -d = -f Cargo.toml -s = -ar | tail -n4
                clap = "2.30.0"
             failure = "0.1.1"
           itertools = "0.7.6"
unicode-segmentation = "1.2.0"
```