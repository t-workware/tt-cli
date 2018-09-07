use clap::ArgMatches;
use tt_core::record::{Record, Local, Date, Datelike, Timelike, TimeZone, Duration};
use tt_core::journal::{Journal, file::{FileJournal, Item}};
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

    pub const LIST: Cmd = Cmd {
        upcase_name: "LIST",
        name: "list",
        short: "",
        desc: "List records"
    };

    pub const DEL: Cmd = Cmd {
        upcase_name: "DEL",
        name: "del",
        short: "",
        desc: "Remove record"
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

    pub const ACTIVITY: Cmd = Cmd {
        upcase_name: "ACTIVITY",
        name: "act",
        short: "",
        desc: "The record duration of activity in minutes"
    };

    pub const REST: Cmd = Cmd {
        upcase_name: "REST",
        name: "rest",
        short: "",
        desc: "The record duration of rest in minutes"
    };

    pub const ALL: Cmd = Cmd {
        upcase_name: "ALL",
        name: "all",
        short: "a",
        desc: "All lines"
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
        self.update(Self::get_offset(matches), |mut record| {
            let note = Self::get_note(matches);
            if let Some(note) = note {
                record.note = note;
            }
            record.update_activity_to_now();
            record
        });
    }

    pub fn restart(&mut self, matches: &ArgMatches) {
        self.update(Self::get_offset(matches), |mut record| {
            let note = Self::get_note(matches);
            if let Some(note) = note {
                record.note = note;
            }
            record.update_rest_to_now();
            record
        });
    }

    pub fn list(&mut self, matches: &ArgMatches) {
        let date = if !Self::is_all(matches) {
            Self::get_date(matches).or(Some(Local::now().date()))
        } else {
            None
        };
        let error_message = format!("Can't list records from journal {:?}", self.journal.path());
        let mut out = Vec::new();

        let mut iter = self.journal.try_iter().expect(&error_message);
        iter.go_to_end();
        loop {
            if let Some(item) = iter.backward(1).get() {
                let line = match item {
                    Item::Record(r) => {
                        if date.is_some() && r.start.is_some() {
                            if r.start.unwrap().date() < date.unwrap() {
                                break;
                            }
                        }
                        r.to_string()
                    },
                    Item::SomeLine(s) => s,
                };
                out.push(line);
            } else {
                break;
            }
        }
        out.iter().rev().for_each(|line| println!("{}", line));
    }

    pub fn del(&mut self, matches: &ArgMatches) {
        let offset = Self::get_offset(matches);
        let error_message = format!("Can't del record in journal {:?}", self.journal.path());
        let print = self.print;

        if !self.journal.remove(&[], Some(offset), |record| {
            if print {
                println!("{}", record.to_string());
            }
            true
        }).expect(&error_message) {
            panic!(error_message);
        }
    }

    pub fn set(&mut self, matches: &ArgMatches) {
        let offset = Self::get_offset(matches);
        if let Some(matches) = matches.subcommand_matches(Cmd::NOTE.name) {
            self.set_note(matches, offset);
        } else if let Some(matches) = matches.subcommand_matches(Cmd::DATE.name) {
            self.set_date(matches, offset);
        } else if let Some(matches) = matches.subcommand_matches(Cmd::ACTIVITY.name) {
            self.set_act(matches, offset);
        } else if let Some(matches) = matches.subcommand_matches(Cmd::REST.name) {
            self.set_rest(matches, offset);
        }
    }

    fn set_note(&mut self, matches: &ArgMatches, offset: i32) {
        self.update(offset, |mut record| {
            if let Some(note) = Self::get_note(matches) {
                record.note = note;
            } else {
                record.note.clear();
            }
            record
        });
    }

    fn set_date(&mut self, matches: &ArgMatches, offset: i32) {
        self.update(offset, |mut record| {
            if let Some(date) = Self::get_date(matches) {
                let hour = record.start.map(|dt| dt.hour()).unwrap_or(0);
                let min = record.start.map(|dt| dt.minute()).unwrap_or(0);
                let sec = record.start.map(|dt| dt.second()).unwrap_or(0);
                record.start = Some(date.and_hms(hour, min, sec));
            }
            record
        });
    }

    fn set_act(&mut self, matches: &ArgMatches, offset: i32) {
        self.update(offset, |mut record| {
            if let Some(act) =  Self::get_act(matches) {
                record.activity = Some(Duration::minutes(act));
            }
            record
        });
    }

    fn set_rest(&mut self, matches: &ArgMatches, offset: i32) {
        self.update(offset, |mut record| {
            if let Some(rest) =  Self::get_rest(matches) {
                record.rest = Some(Duration::minutes(rest));
            }
            record
        });
    }

    fn update<F>(&mut self, offset: i32, f: F)
        where F: FnOnce(Record) -> Record,
    {
        let error_message = format!("Can't update record in journal {:?}", self.journal.path());
        let print = self.print;

        if !self.journal.update(&[], Some(offset), |record| {
            let record = f(record);
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

    fn get_date(matches: &ArgMatches) -> Option<Date<Local>> {
        matches.args
            .get(Cmd::DATE.upcase_name)
            .map(|arg| {
                let arg = arg.vals[0]
                    .clone()
                    .into_string()
                    .expect(&format!("Can't convert date {:?} to UTF-8 string", arg.vals[0]));
                let now = Local::now();

                if  &arg == "now" {
                    now.date()
                } else {
                    let mut items: Vec<i32> = arg
                        .split('-')
                        .map(|s| s.parse().expect(&format!("Can't convert part of date {:?} to i32", s)))
                        .collect::<Vec<i32>>();
                    if items.len() < 1 || items.len() > 3 {
                        panic!("Can't convert date {:?} to Date<Local>", arg);
                    }
                    items.reverse();

                    let day = items[0] as u32;
                    let month = if items.len() > 1 {
                        items[1] as u32
                    } else {
                        now.month()
                    };
                    let year = if items.len() > 2 {
                        items[2]
                    } else {
                        now.year()
                    };
                    Local.ymd(year, month, day)
                }
            })
    }

    fn get_act(matches: &ArgMatches) -> Option<i64> {
        matches.args
            .get(Cmd::ACTIVITY.upcase_name)
            .map(|arg|
                arg.vals[0]
                    .clone()
                    .into_string()
                    .expect(&format!("Can't convert duration of activity {:?} to UTF-8 string", arg.vals[0]))
                    .parse::<i64>()
                    .expect(&format!("Can't convert duration of activity {:?} to i64 number", arg.vals[0]))
            )
    }

    fn get_rest(matches: &ArgMatches) -> Option<i64> {
        matches.args
            .get(Cmd::REST.upcase_name)
            .map(|arg|
                arg.vals[0]
                    .clone()
                    .into_string()
                    .expect(&format!("Can't convert duration of rest {:?} to UTF-8 string", arg.vals[0]))
                    .parse::<i64>()
                    .expect(&format!("Can't convert duration of rest {:?} to i64 number", arg.vals[0]))
            )
    }

    fn is_all(matches: &ArgMatches) -> bool {
        matches.occurrences_of(Cmd::ALL.name) > 0
    }
}
