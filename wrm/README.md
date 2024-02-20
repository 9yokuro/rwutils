# Wrm - Remove files or directories

## Installation
Run the following Cargo command:
```
cargo install wrm
```
Or download prebuilt binary from the [GitHub release page](https://github.com/9yokuro/wrm/releases)

## Usage
To move files to trash(`$HOME/.config/wrm/trash`):
```console
wrm remove foo bar ...
```

To delete files:
```console
wrm delete foo bar ...
```

To restore files in trash to where they came from:
```console
wrm restore ~/.config/wrm/trash/foo ~/.config/wrm/trash/bar ...
```

To list all files and directories in trash:
```console
wrm list
```

To delete all files and directories in trash permanently:
```console
wrm empty
```

### Options
```console
-n, --noninteractive  Do not prompt whether to change destinations
-q, --quiet           Do not print log messages
-h, --help            Print help
-V, --version         Print version
```
