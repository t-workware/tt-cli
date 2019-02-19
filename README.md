# Time tracker CLI

This is a time tracking console tool. Usage example:

```
$ tt start
[2018-10-11 18:34:56, ]

$ tt stop Day work task2
[2018-10-11 18:34:56, 45] Day work task2

$ tt restart
[2018-10-11 18:34:56, 45 (5)] Day work task2

$ tt stop
[2018-10-11 18:34:56, 52 (5)] Day work task2

$ tt list
[2018-10-11 10:21:13, 17] Some work
[2018-10-11 13:48:02, 15] Day work task1
[2018-10-11 18:34:56, 52 (5)] Day work task2

$ tt report
67  Day work
  15  task1
  52  task2
17  Some work
---------
Total: 84
```

## Installation

To build `tt-cli` from source, you will need to have Git, [Rust and Cargo](https://www.rust-lang.org) installed.

From a terminal, you can now run following commands:

```
$ git clone https://github.com/t-workware/tt-cli.git
$ cd tt-cli
$ cargo build --release
```

The binary will be available in the `target/release` directory. To copy executable to `/usr/bin`, run the following:

```
$ sudo cp target/release/tt-cli /usr/bin/tt
```

The tool requires a config file with name `tt-cli.toml`, located in TT_CLI_HOME directory. You need to set this environment variable too:

1. Add the following line to the file `~/.bashrc`:
```
export TT_CLI_HOME=~/.tt
```
2. Create the tt-cli home directory:
```
$ mkdir ~/.tt
```
3. Add the `tt-cli.toml` config file with a line specifying the path to the time journal file:
```
$ echo 'journal_file = "/path/to/journal.txt"' > ~/.tt/tt-cli.toml
```

To check run the following command:

```
$ tt --version
```

## Updating

To update, run following commands in the `tt-cli` local repository folder:

```
$ git pull origin master
$ cargo build --release
$ sudo rm /usr/bin/tt
$ sudo cp target/release/tt-cli /usr/bin/tt
```

## Usage

All available commands are listed in the help:

```
$ tt -h
```
```
USAGE:
    tt-cli [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    del        Remove record
    help       Prints this message or the help of the given subcommand(s)
    list       List records
    report     Generate and display report
    restart    Resume paused tracking
    set        Setup record attribute
    start      Start tracking
    stop       Stop tracking
```

Each command has its own help:
```
$ tt [SUBCOMMAND] -h
```

### Examples

1. Start a new tracking:
```
$ tt start
$ tt start Record note
```

2. Stop last tracking:
```
$ tt stop
$ tt stop New record note
```

3. Stop last but one tracking:
```
$ tt stop -n 1
$ tt stop -n 1 New last but one record note
```

4. Restart stopped tracking:
```
$ tt restart
$ tt restart New restarted note
$ tt restart -n 1 New last but one restarted note
```

5. List records:
```
$ tt list
$ tt list 2018-12-01
$ tt list 12-01
$ tt list 01
```

6. Generate and show report:
```
$ tt report
$ tt report 2018-12-01
$ tt report 12-01
$ tt report 01
```

7. Setup record attributes:
```
$ tt set note New last record note
$ tt set act 45
$ tt set rest 5
$ tt set date 2018-12-03
$ tt set time 12:25
$ tt set -n 2 note New some record note
```

8. Remove record:
```
$ tt del
$ tt del -n 1
```