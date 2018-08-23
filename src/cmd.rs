//use clap::ArgMatches;
//use failure::Error;
//use lang::{OsStrX, Str};
//use settings::{Settings, Setup};

#[derive(Default)]
pub struct Cmd {
    pub upcase_name: &'static str,
    pub name: &'static str,
    pub short: &'static str,
    pub desc: &'static str,
}

impl Cmd {
    pub const START: Cmd = Cmd {
        upcase_name: "START",
        name: "start",
        short: "",
        desc: "Start tracking"
    };

    pub const STOP: Cmd = Cmd {
        upcase_name: "STOP",
        name: "stop",
        short: "",
        desc: "Stop tracking"
    };

    pub const RESTART: Cmd = Cmd {
        upcase_name: "RESTART",
        name: "restart",
        short: "",
        desc: "Resume paused tracking"
    };

    pub const SET: Cmd = Cmd {
        upcase_name: "SET",
        name: "set",
        short: "",
        desc: "Setup record attribute"
    };

    pub const NOTE: Cmd = Cmd {
        upcase_name: "NOTE",
        name: "note",
        short: "",
        desc: "The record note"
    };

    pub const DATE: Cmd = Cmd {
        upcase_name: "DATE",
        name: "date",
        short: "",
        desc: "The record start date, for example: \"2018-08-25\""
    };

    pub const TIME: Cmd = Cmd {
        upcase_name: "TIME",
        name: "time",
        short: "",
        desc: "The record start time, for example: \"14:09:21\""
    };

    pub const DATETIME: Cmd = Cmd {
        upcase_name: "DATETIME",
        name: "datetime",
        short: "",
        desc: "The record start datetime, for example: \"2018-08-25 14:09:21\""
    };

    pub const DURATION: Cmd = Cmd {
        upcase_name: "DURATION",
        name: "dur",
        short: "",
        desc: "The record duration in minutes"
    };

    pub const CORRECTION: Cmd = Cmd {
        upcase_name: "CORRECTION",
        name: "cor",
        short: "",
        desc: "The record correction in minutes"
    };

    pub const OFFSET: Cmd = Cmd {
        upcase_name: "OFFSET",
        name: "offset",
        short: "n",
        desc: "Backward offset from the last record"
    };
}
