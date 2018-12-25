#[macro_use]
extern crate file_assertions;
extern crate tt_core;

#[macro_use]
mod common;

use tt_core::record::Record;
use tt_core::journal::Journal;
use common::TestPaths;

#[test]
fn start_record() {
    let test_paths = TestPaths::new("test_start");
    let journal = test_paths.init();
    let journal_file = test_paths.journal_file();

    run!("tt-cli start record1");

    let record = journal.get(&[], None)
        .expect(&format!("Can't get record from {:?}", journal_file))
        .expect(&format!("The record in {:?} is empty", journal_file));
    assert!(record.start.is_some());
    let expected = format!(
        "[{}, ] record1\n",
        record.start.unwrap().format(Record::START_DATETIME_FORMAT)
    );
    assert_content!(journal_file, expected);

    run!("tt-cli start \"record 2\"");

    let record2 = journal.get(&[], Some(1))
        .expect(&format!("Can't get record from {:?}", journal_file))
        .expect(&format!("The record in {:?} is empty", journal_file));
    assert!(record2.start.is_some());
    let expected = format!(
        "[{}, ] record1\n[{}, ] record 2\n",
        record.start.unwrap().format(Record::START_DATETIME_FORMAT),
        record2.start.unwrap().format(Record::START_DATETIME_FORMAT)
    );
    assert_content!(journal_file, expected);

    run!("tt-cli start record 3");

    let record3 = journal.get(&[], Some(2))
        .expect(&format!("Can't get record from {:?}", journal_file))
        .expect(&format!("The record in {:?} is empty", journal_file));
    assert!(record3.start.is_some());
    let expected = format!(
        "[{}, ] record1\n[{}, ] record 2\n[{}, ] record 3\n",
        record.start.unwrap().format(Record::START_DATETIME_FORMAT),
        record2.start.unwrap().format(Record::START_DATETIME_FORMAT),
        record3.start.unwrap().format(Record::START_DATETIME_FORMAT)
    );
    assert_content!(journal_file, expected);
}
