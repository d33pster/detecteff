# Overview

[ [Detecteff](https://crates.io/crates/detecteff) ] (DetectF or Detective) is a command-line utility to find duplicate files in a directory.

## Features

- Optional Recursive Scan
- Default Output Format or a better readability output format
- Thorough

## Installation

In terminal run:

```console
$ cargo install detecteff
  Installing detecteff v0.1.0
    Updating crates.io index
   Compiling libc v0.2.154
   Compiling option-ext v0.2.0
   Compiling dirs-sys v0.4.1
   Compiling dirs v5.0.1
   Compiling rustypath v0.1.1
   Compiling argrust v0.1.0
   Compiling detecteff v0.1.0
    Finished `release` profile [optimized] target(s) in 8.66s
  Installing /Users/XXXXX/.cargo/bin/detectf
   Installed package `detecteff v0.1.0 (/Users/XXXXX/detecteff)` (executable `detectf`)
```

## Usage

For help, run in terminal:

```console
$ detectf --help
detecteff help
   -
   [INFO]
   | -h, --help : show help text and exit.
   | -v, --version : show version and exit.
   -
   [FLAG]
   | -r, --recursive : recursive mode. Default -> OFF
   | -f, --format : show formatted output. Default -> OFF
   -
   [INPUT]
   | -s, --scan <directory> : scan the directory for duplicate files.
```

## Example Usages

```bash
detectf --scan ~/ -r -f
```

```bash
detectf --scan ~/
```

```bash
detectf --scan ./target -r
```
