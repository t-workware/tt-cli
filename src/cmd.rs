use clap::ArgMatches;
use tt_core::record::Record;
use tt_core::journal::{Journal, file::FileJournal};
use settings::Settings;

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

pub struct CmdProcessor {
    journal: FileJournal,
    print: bool,
}

impl CmdProcessor {
    pub fn new(settings: &Settings) -> Self {
        CmdProcessor {
            journal: FileJournal::new(&settings.journal_file),
            print: settings.print,
        }
    }

    pub fn start(&mut self, matches: &ArgMatches) {
        let mut record = Record::now();

        if let Some(note) = Self::get_note(matches) {
            record.note = note;
        }
        self.journal.add(&record)
            .expect(&format!("Can't add new record to journal {:?}", self.journal.path()));
        if self.print {
            println!("{}", record.to_string());
        }
    }

    pub fn stop(&mut self, matches: &ArgMatches) {
        let offset = Self::get_offset(matches);
        let note = Self::get_note(matches);
        let error_message = format!("Can't update last record in journal {:?}", self.journal.path());
        let print = self.print;

        if !self.journal.update(&[], Some(offset), |mut record| {
            if let Some(note) = note {
                record.note = note;
            }
            record.set_duration_to_now();
            if print {
                println!("{}", record.to_string());
            }
            Some(record)
        }).expect(&error_message) {
            panic!(error_message);
        }
    }

    pub fn restart(&mut self, matches: &ArgMatches) {
        let offset = Self::get_offset(matches);
        let note = Self::get_note(matches);
        let error_message = format!("Can't update last record in journal {:?}", self.journal.path());
        let print = self.print;

        if !self.journal.update(&[], Some(offset), |mut record| {
            if let Some(note) = note {
                record.note = note;
            }
            record.set_correction_to_now();
            if print {
                println!("{}", record.to_string());
            }
            Some(record)
        }).expect(&error_message) {
            panic!(error_message);
        }
    }

    fn get_offset(matches: &ArgMatches) -> i32 {
        matches.args
            .get(Cmd::OFFSET.name)
            .map(|arg|
                arg.vals[0]
                    .clone()
                    .into_string()
                    .expect(&format!("Can't convert offset {:?} to UTF-8 string", arg.vals[0]))
                    .parse::<i32>()
                    .map(|n| -n - 1)
                    .expect(&format!("Can't convert offset {:?} to i32 number", arg.vals[0]))
            )
            .unwrap_or(-1)
    }

    fn get_note(matches: &ArgMatches) -> Option<String> {
        matches.args
            .get(Cmd::NOTE.upcase_name)
            .map(|arg|
                arg.vals
                    .iter()
                    .map(|val|
                        val.clone().into_string().expect(&format!("Can't convert note {:?} to UTF-8 string", arg.vals))
                    )
                    .collect::<Vec<_>>()
                    .join(" ")
            )
    }
}
