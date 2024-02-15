# Wmk - A file creation tool written in rust

![Crates.io Version](https://img.shields.io/crates/v/wmk)

## Installation
Run the following Cargo command:
```console
cargo install wmk
```

## Usage
To create files:
```console
wmk foo.txt bar.txt ...
```

To create directories:
```console
wmk --directory foo_dir bar_dir ...
```

### Options
```console
-d, --directory  Create directories instead of files
-q, --quiet      Do not print log messages
-h, --help       Print help
-V, --version    Print version
```
