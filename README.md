# playrs

*Call the interface of rust playground to compile and run the rust code.*

## Install

`cargo install playrs`

## Usage

Enter `playrs - h` on the command line to view the detailed usage commands:
```
Play Rust 0.1.0
Compile and run your rust code

USAGE:
    playrs.exe [FLAGS] [OPTIONS] <file>

FLAGS:
    -b, --backtrace    Whether to enable backtrace
    -h, --help         Prints help information
    -t, --tests        Whether it is a test
    -V, --version      Prints version information

OPTIONS:
    -c, --channel <channel>              Compile channel: stable, nightly or beta [default: stable]
    -e, --edition <edition>              Compile edition: 2015, 2018 or 2021 [default: 2021]
    -m, --mode <mode>                    Compile mode: debug or release [default: debug]
    -p, --program-type <program-type>    Crate type: bin or lib [default: bin]

ARGS:
    <file>    rust code file
```

## Example

`playrs rustfile.rs`

## LICENSE

This project is licensed under the MIT License (see the LICENSE file for details).