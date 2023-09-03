# Grace

[![Crates info](https://img.shields.io/crates/v/grace-cli.svg)](https://crates.io/crates/grace-cli)

<img src="misc/grace-logo.png" width="209" height="269" />  

_Image author: [Мельк](https://www.linkedin.com/in/katemillart/)_

**CLI tool for processing files and strings.**

Grace helps you organize your files by converting them to the proper case and clearing out unnecessary characters such as ` ' [ ] and many others.

Grace can also rename strings to the correct case and clear them of unnecessary characters.

Now Grace can renames files, directories and strings into a lot of cases, such as

- `Alternating` -> tHiS wAy
- `Snake` -> this_way
- `Camel` -> thisWay
- `Kebab` -> this-way
- `Dot` -> this.way
- `Header` -> This-Way
- `Normal` -> this way
- `Original` -> "this way" (returns the string as it was)
- `Pascal` -> ThisWay
- `Path` -> this/way
- `Sentence` -> This way
- `Title` -> This Way
- `UpperSnake` -> THIS_WAY
- `WindowsPath` -> this\way

## How To

### How to install Grace

You can download suitable executable from https://github.com/meinbaumm/grace/releases and copy it in some directory that is listed in your PATH (e.g. ~/bin).

You may also use cargo to install grace from crates.io:

`cargo install grace-cli`

### How to build Grace

Grace is written in Rust. You may build it yourself with the help of cargo. Just clone this repository and execute the cargo build command in its main directory:

`cargo build --release`

### How to use grace instead of grace-cli

Grace now has the name `grace-cli` in crates.io.
But I prefer just `grace` more.
To avoid writing `grace-cli` every time, you can add an alias to your .bashrc or .zshrc file.

If you installed grace with cargo:

```bash
which grace-cli
```

Then copy the output and in your .bashrc or .zshrc file write

```bash
alias grace="~/./.cargo/bin/grace-cli"
```

Next source file like. And use with pleasure :)

```bash
source .zshrc
```

## Command overview

### Help command

```bash
grace-cli --help
```

```
CLI tool for processing files and strings.

Usage: grace-cli <COMMAND>

Commands:
  recase    Recase string, file, or directory of files
  sanitize  Sanitize string of unnecessary characters
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Recase file

```bash
grace-cli recase file -f example\ file\ \[2023\].pdf -i snake --sanitize
```

Will recase `example file [2023].pdf` file into snake case `example_file_2023.pdf`, removing redundant symbols.

### Recase many files

```bash
grace-cli recase files -d test-dir/ -i camel
```

Will recase all files in provided directory into camelCase. If you wand recase only files with specific extension, use `-f/--formats` flag.

```bash
grace-cli recase files -d test-dir/ -i camel -f "pdf, epub"
```

If you wand recase directories too, you can write

```bash
grace-cli recase files -d test-dir/ -i camel -d
```

#### Rename files recursively

Grace also can rename files recursively, just provide `-r/--recursive` flag.

```bash
grace-cli recase files -d test-dir/ -i camel -r
```

Will recase all files in provided directories and files in subdirectories.

> Unfortunately, recursively renaming directories is not supported now.

### Recase directory

```bash
grace-clie recase dir -f test-dir/ -i snake
```

Will recase `test-dir/` into snake case.

### Recase strings

```bash
grace-cli recase string "Some Long String You Want To Recase Into Sentence" -i sentence
```

Will return

```
Some long string you want to recase into sentence
```
