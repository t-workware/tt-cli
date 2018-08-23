#[macro_use]
extern crate file_assertions;
extern crate tt_core;

#[macro_use]
mod common;

use std::path::PathBuf;
use std::env;

use tt_core::record::Record;
use tt_core::journal::{Journal, file::FileJournal};

#[test]
fn start_record() {
    let test_dir = &["..", "target", "test_start"].iter().collect::<PathBuf>();
    let journal_file = &test_dir.join("journal.txt");
    let config_file = &test_dir.join("tt-cli.toml");
    let config_content = format!("journal_file = {:?}", journal_file.as_os_str());

    clear_dir!(test_dir);
    create_file!(config_file, config_content);
    env::remove_var("TT_CLI_HOME");
    env::set_var("TT_CLI_CONFIG_FILE_NAME", config_file);
    let journal = FileJournal::new(journal_file);

    run!("tt-cli start record1");

    let record = journal.get(&[], None)
        .expect(&format!("Can't get record from {:?}", journal_file))
        .expect(&format!("The record in {:?} is empty", journal_file));
    assert!(record.start.is_some());
    let expected = format!(
        "[{},  ()] record1\n",
        record.start.unwrap().format(Record::START_DATETIME_FORMAT).to_string()
    );
    assert_content!(journal_file, expected);

    run!("tt-cli start \"record 2\"");

    let record2 = journal.get(&[], Some(1))
        .expect(&format!("Can't get record from {:?}", journal_file))
        .expect(&format!("The record in {:?} is empty", journal_file));
    assert!(record2.start.is_some());
    let expected = format!(
        "[{},  ()] record1\n[{},  ()] record 2\n",
        record.start.unwrap().format(Record::START_DATETIME_FORMAT).to_string(),
        record2.start.unwrap().format(Record::START_DATETIME_FORMAT).to_string()
    );
    assert_content!(journal_file, expected);
}
