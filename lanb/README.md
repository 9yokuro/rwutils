# lanb - A cli tool to create symbolic links and hard links.

![example](./images/example1.png)

## Features
- Automatically convert relative path to absolute path before create links.

## Installation
Run the following Cargo command:
```
cargo install lanb
```
Or download prebuilt binary from the [GitHub release page](https://github.com/p1486/lanb/releases)

## Usage
To create symbolic link `foo` -> `bar`:
```
lanb foo bar
```
If bar is a directory, this will create `foo` -> `bar/the_name_of_foo`.

If you specified three files or more, create symbolic link `nth` -> `last/name_of_nth_file`:
```
lanb foo bar baz
```
If baz is not a directory, return error.

### Options
- `-b`, `--backup` - Make backup of each existing destination file
- `-s`, `--suffix <SUFFIX>` - Override the usual backup suffix [defalt: ~]
- `-n`, `--noninteractive` - Do not prompt whether to remove destinations
- `-H`, `--hardlink` - Make hard links instead of symbolic links
- `-q`, `--quiet` - Do not print name of each linked file

## License
This project is licensed under the MIT License and the Apache-2.0.
