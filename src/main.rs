extern crate clap;
extern crate config;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate tt_core;

mod cmd;
mod settings;

use clap::{App, Arg, SubCommand};
use tt_core::record::Record;
use tt_core::journal::{Journal, file::FileJournal};

use cmd::Cmd;
use settings::Settings;

const VERSION: &'static str = "0.1.0"; // Related with `version` value in Cargo.toml

fn main() {
    let settings = Settings::new().expect("Read settings error");
    let matches = App::new("TimeTracker CLI")
        .version(VERSION)
        .about("The command line interface of TimeTracker")
        .subcommand(SubCommand::with_name(Cmd::START.name)
            .about(Cmd::START.desc)
            .arg(Arg::with_name(Cmd::NOTE.upcase_name)
                .help(Cmd::NOTE.desc)))
        .subcommand(SubCommand::with_name(Cmd::STOP.name)
            .about(Cmd::STOP.desc)
            .arg(Arg::with_name(Cmd::OFFSET.name)
                .short(Cmd::OFFSET.short)
                .long(Cmd::OFFSET.name)
                .value_name(Cmd::OFFSET.upcase_name)
                .help(Cmd::OFFSET.desc)
                .takes_value(true))
            .arg(Arg::with_name(Cmd::NOTE.upcase_name)
                .help(Cmd::NOTE.desc)))
        .subcommand(SubCommand::with_name(Cmd::SET.name)
            .about(Cmd::SET.desc)
            .arg(Arg::with_name(Cmd::OFFSET.name)
                .short(Cmd::OFFSET.short)
                .long(Cmd::OFFSET.name)
                .value_name(Cmd::OFFSET.upcase_name)
                .help(Cmd::OFFSET.desc)
                .takes_value(true))
            .subcommand(SubCommand::with_name(Cmd::NOTE.name)
                .about(Cmd::NOTE.desc)
                .arg(Arg::with_name(Cmd::NOTE.upcase_name)
                    .help(Cmd::NOTE.desc)))
            .subcommand(SubCommand::with_name(Cmd::DATE.name)
                .about(Cmd::DATE.desc)
                .arg(Arg::with_name(Cmd::DATE.upcase_name)
                    .help(Cmd::DATE.desc)))
            .subcommand(SubCommand::with_name(Cmd::TIME.name)
                .about(Cmd::TIME.desc)
                .arg(Arg::with_name(Cmd::TIME.upcase_name)
                    .help(Cmd::TIME.desc)))
            .subcommand(SubCommand::with_name(Cmd::DATETIME.name)
                .about(Cmd::DATETIME.desc)
                .arg(Arg::with_name(Cmd::DATETIME.upcase_name)
                    .help(Cmd::DATETIME.desc)))
            .subcommand(SubCommand::with_name(Cmd::DURATION.name)
                .about(Cmd::DURATION.desc)
                .arg(Arg::with_name(Cmd::DURATION.upcase_name)
                    .help(Cmd::DURATION.desc)))
            .subcommand(SubCommand::with_name(Cmd::CORRECTION.name)
                .about(Cmd::CORRECTION.desc)
                .arg(Arg::with_name(Cmd::CORRECTION.upcase_name)
                    .help(Cmd::CORRECTION.desc))))
        .get_matches();

    let mut journal = FileJournal::new(&settings.journal_file);
    if let Some(matches) = matches.subcommand_matches(Cmd::START.name) {
        let mut record = Record::now();
        if let Some(arg) = matches.args.get(Cmd::NOTE.upcase_name) {
            record.note = arg.vals[0].clone().into_string()
                .expect(&format!("Can't convert note {:?} to UTF-8 string", arg.vals[0]));
        }
        journal.add(&record)
            .expect(&format!("Can't add new record to journal {:?}", journal.path()));
    } else if let Some(matches) = matches.subcommand_matches(Cmd::STOP.name) {
        let offset = matches.args
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
            .unwrap_or(-1);
        let note = matches.args
            .get(Cmd::NOTE.upcase_name)
            .map(|arg|
                arg.vals[0]
                    .clone()
                    .into_string()
                    .expect(&format!("Can't convert note {:?} to UTF-8 string", arg.vals[0]))
            );
        let error_message = format!("Can't update last record in journal {:?}", journal.path());
        if !journal.update(&[], Some(offset), |mut record| {
            if let Some(note) = note {
                record.note = note;
            }
            record.set_duration_to_now();
            Some(record)
        }).expect(&error_message) {
            panic!(error_message);
        }
    }
}
