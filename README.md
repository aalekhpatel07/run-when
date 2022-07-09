# run-when

Monitor filesystem for changes and run a specific executable some time after the changes are announced.

## Usage

```sh
run-when --help
```

```
run-when 1.0.0
Aalekh Patel
Run a (debounced) command upon changes to the filesystem.

USAGE:
    run-when [OPTIONS] --file <FILE> --command-file <COMMAND_FILE>

OPTIONS:
    -c, --command-file <COMMAND_FILE>
            An executable to run once a change is detected

    -f, --file <FILE>
            The file/directory to watch. If a directory is specified, will watch all files in it
            (but not recursively, unless -r is also specified)

    -h, --help
            Print help information

    -r, --recursive
            Whether to watch a directory recursively

    -t, --debounce-period <DEBOUNCE_PERIOD>
            The debounce period (i.e. wait for a duration of X before running the specified
            executable) [default: 600ms]

    -V, --version
            Print version information
```

### Examples

- Watch the `src` directory for changes and once every `600ms` run the build script stored in `build.sh`.

Suppose the contents of `build.sh` are:

```sh
#!/usr/bin/sh

cargo build --release
```

Then we can run the following to start a watcher on the `src` directory and all children files recursively.
```sh
run-when --file src --recursive --command-file ./build.sh
```


