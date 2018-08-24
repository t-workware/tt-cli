#[macro_use]
extern crate file_assertions;
extern crate tt_core;

#[macro_use]
mod common;

use std::path::PathBuf;
use std::env;

use tt_core::record::{Record, Duration};
use tt_core::journal::{Journal, file::FileJournal};

#[test]
fn stop_record() {
    let test_dir = &["..", "target", "test_stop"].iter().collect::<PathBuf>();
    let journal_file = &test_dir.join("journal.txt");
    let config_file = &test_dir.join("tt-cli.toml");
    let config_content = format!("journal_file = {:?}", journal_file.as_os_str());

    clear_dir!(test_dir);
    create_file!(config_file, config_content);
    env::remove_var("TT_CLI_HOME");
    env::set_var("TT_CLI_CONFIG_FILE_NAME", config_file);
    let journal = FileJournal::new(journal_file);

    let mut record = Record::now();
    record.start.as_mut().map(|start| *start = *start - Duration::minutes(12));
    let start = record.start.unwrap().format(Record::START_DATETIME_FORMAT);

    let content = format!("[{},  ()]", start);
    create_file!(journal_file, content);

    run!("tt-cli stop record1");
    let expected = format!("[{}, 12 ()] record1\n", start);
    assert_content!(journal_file, expected);

    let content = format!("[{}, 12 ()] record1\n[{},  ()]\n", start, start);
    create_file!(journal_file, content);

    run!("tt-cli stop record 2");
    let expected = format!("[{}, 12 ()] record1\n[{}, 12 ()] record 2\n", start, start);
    assert_content!(journal_file, expected);

    let content = format!("[{}, 12 ()] record1\n[{}, 12 ()] record 2\n[{},  ()] record 3\n", start, start, start);
    create_file!(journal_file, content);

    run!("tt-cli stop");
    let expected = format!("[{}, 12 ()] record1\n[{}, 12 ()] record 2\n[{}, 12 ()] record 3\n", start, start, start);
    assert_content!(journal_file, expected);

    let content = format!("[{},  ()] \n[{},  ()]\n[{},  ()]\n", start, start, start);
    create_file!(journal_file, content);

    run!("tt-cli stop -n 2 record1");
    let expected = format!("[{}, 12 ()] record1\n[{},  ()]\n[{},  ()]\n", start, start, start);
    assert_content!(journal_file, expected);

    run!("tt-cli stop -n 1");
    let expected = format!("[{}, 12 ()] record1\n[{}, 12 ()]\n[{},  ()]\n", start, start, start);
    assert_content!(journal_file, expected);

    run!("tt-cli stop -n 0 record 3");
    let expected = format!("[{}, 12 ()] record1\n[{}, 12 ()]\n[{}, 12 ()] record 3\n", start, start, start);
    assert_content!(journal_file, expected);

    let first_record = journal.get(&[], None)
        .expect(&format!("Can't get record from {:?}", journal_file))
        .expect(&format!("The record in {:?} is empty", journal_file));
    record.duration = Some(Duration::minutes(12));
    record.note = "record1".to_string();
    assert_eq!(record, first_record);
}
