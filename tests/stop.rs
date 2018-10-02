#[macro_use]
extern crate file_assertions;
extern crate tt_core;

#[macro_use]
mod common;

use tt_core::record::{Record, Duration};
use tt_core::journal::Journal;
use common::TestPaths;

#[test]
fn stop_record() {
    let test_paths = TestPaths::new("test_stop");
    let journal = test_paths.init();
    let journal_file = test_paths.journal_file();

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
    record.activity = Some(Duration::minutes(12));
    record.note = "record1".to_string();
    assert_eq!(record, first_record);
}
