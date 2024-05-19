# Overview

[ [Detecteff](https://crates.io/crates/detecteff) ] (DetectF or Detective) is a command-line utility to find duplicate files in a directory.

## Badges

![Crates.io Total Downloads](https://img.shields.io/crates/d/detecteff)
![GitHub License](https://img.shields.io/github/license/d33pster/detecteff)
![Libraries.io SourceRank](https://img.shields.io/librariesio/sourcerank/cargo/detecteff)
![Crates.io Version](https://img.shields.io/crates/v/detecteff)

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [HELP](#help)
- [FIX Windows Terminal for ANSI Colours](#fix-windows-terminal-for-ansi-colours)
  - [Using `curl`](#using-curl)
  - [Using `wget`](#using-wget)
  - [Manually](#manually)
- [Usage](#usage)

## Features

- Optional Recursive Scan
- Default Output Format or a better readability output format
- Thorough
- Super-fast after v0.3.0
- Ability to ignore directories or sub directories.
- Automatic ignores Directories whose name start with `.` as they are not to be messed with.

**NOTE:** If scanning the `HOME` directory of your OS, be careful as some directories shouldn't be messed with like the `Library` and `Applications` folder in macOS. Try scanning individual directories in the home directory.

**For Example**  

Suppose this is your Directory structure for `HOME` directory.

```console
~/--- |
     abc.txt
     dir1/--- |
             xyz.txt
     123.txt
     dir2/--- |
             456.txt
     hehe.txt
     hello.txt
     ...
```

Do not scan the `HOME` directory directly. Try scanning the individual directories instead.

OR if you really need to, try the `--ignore` flag to mention directories to ignore (case insensitive).

**ADDITIONAL NOTE:**

- Avoid scanning OS directories or any application installation directory or else it might result in tampering with important files.
- Before using `--delete` or `-d` flag to delete the temp files, check the list of files that will be deleted (white background, red foreground) that will be printed after scanning.

## Installation

In terminal run:

```console
$ cargo install detecteff
  Installing detecteff v0.3.1
    Updating crates.io index
   Compiling libc v0.2.155
   Compiling option-ext v0.2.0
   Compiling colorized v1.0.0
   Compiling dirs-sys v0.4.1
   Compiling dirs v5.0.1
   Compiling rustypath v0.1.1
   Compiling argrust v0.1.0
   Compiling detecteff v0.3.1 (/Users/XXXXXX/detecteff)
    Finished `release` profile [optimized] target(s) in 2.02s
```

## HELP

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
   | -s, --scan <directory> : scan the directory for duplicate files. (Mandatory)
   | -i, --ignore <directory1>, <directory2>, ... : ignore these directories. (Optional)
   -
   [IRREVERSIBLE FLAG]
   | -d, --delete : delete any found duplicates. Default -> OFF
```

## FIX Windows Terminal for ANSI Colours

By default, Windows terminal doesn't support ANSI colour codes which are implemented in `v0.3.1`. You can fix this by:

Run the following command to download the `fix-terminal.bat` file from the `detecteff` repository.

### Using `curl`

```bash
curl -o fix-terminal.bat https://raw.githubusercontent.com/d33pster/detecteff/main/fix-terminal.bat
```

### Using `wget`

```bash
wget -O fix-terminal.bat https://raw.githubusercontent.com/d33pster/detecteff/main/fix-terminal.bat
```

### Manually

Go to [`https://github.com/d33pster/detecteff`](https://github.com/d33pster/detecteff) and click on the `fix-terminal.bat` file and then click the download button.

---

Double Click the `fix-terminal.bat` to check and fix the Windows Registry for ANSI colour support in CMD.

## Usage

`Deteceff` can be used to scan for duplicated files that were left behind by either user, programs or because you downloaded some file again 'cause you weren't able to find the first copy of the file. Whatever it may be, `Detecteff` is the solution.

**_NOTE_:** The longer arguments are used here, but feel free to use the short versions or a combination of long or short arguments as you please. To implement this argument parsing in your own project, checkout [`argrust`](https://crates.io/crates/argrust)

> Search for duplicate files in a directory:

```bash
detectf --scan <directory>
```

> Search for duplicate files in a directory tree (recursive):

```bash
detectf --scan <directory> --recursive
```

> Show the output in a formatted manner:

```bash
detectf --scan <directory> --recursive --format
```

> DELETE the found duplicated files. (need to scan again as of `v0.3.1`. Will be fixed in `v0.3.2`)

```bash
detectf --scan <directory> --delete
```

> For help:

```bash
detectf --help
```

> For version:

```bash
detectf --version
```
