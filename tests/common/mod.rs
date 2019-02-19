#[macro_export]
macro_rules! target_path {
    ($path:tt) => {
        &format!("target/debug/{}", $path)
    };
}

#[macro_export]
macro_rules! run {
    ($($command:tt),*) => {
        $(
            {
                use std::process::Command;

                let args = crate::common::split_args($command);

                let cmd = &args[0];
                let mut cmd = Command::new(target_path!(cmd));
                for arg in args[1..].iter() {
                    cmd.arg(&arg);
                }
                println!("command: {:?}", cmd);
                let output = cmd.output().expect(&format!("Failed execute command `{}`", $command));
                println!("{}", output.status);
                println!("{}", String::from_utf8_lossy(&output.stdout));
                println!("{}", String::from_utf8_lossy(&output.stderr));
            }
        )*
    };
}

#[macro_export]
macro_rules! assert_output {
    ($([$($command:tt),*] => $out:tt),*) => {
        $($(
            assert_output!($command => $out);
        )*)*
    };
    ($($command:tt => $out:tt),*) => {
        $(
            {
                use std::process::Command;

                let args = crate::common::split_args($command);
                let cmd = &args[0];
                let mut cmd = Command::new(target_path!(cmd));
                for arg in args[1..].iter() {
                    cmd.arg(&arg);
                }
                println!("command: {:?}", cmd);
                let output = cmd.output().expect(&format!("Failed execute command `{}`", $command));
                println!("{}", output.status);
                let stdout = String::from_utf8_lossy(&output.stdout);
                println!("{}", stdout);
                println!("{}", String::from_utf8_lossy(&output.stderr));

                let outs: Vec<_> = $out.trim().split("\n").collect();
                let stdouts: Vec<_> = stdout.trim().split("\n").collect();

                assert_eq!(outs.len(), stdouts.len(), "\n  left: {:?}\n right: {:?}", outs, stdouts);

                for out in outs.iter() {
                    assert!(stdouts.contains(out), "`{}` is not in {:?}", out, stdouts);
                }
            }
        )*
    };
}

pub fn split_args(line: &str) -> Vec<String> {
    let mut args = vec![];
    let mut start = 0;
    let mut quote_bch = 0;

    let push_arg = |args: &mut Vec<_>, slice| {
        let arg = String::from_utf8_lossy(slice).to_string().replace("\"", "");
        if !arg.is_empty() {
            args.push(arg);
        }
    };

    for (index, &bch) in line.as_bytes().iter().enumerate() {
        match bch {
            b' ' | b'\t' | b'\n' | b'\r' => {
                if quote_bch == 0 {
                    push_arg(&mut args, &line.as_bytes()[start..index]);
                    start = index + 1;
                }
            }
            b'"' | b'\'' => {
                if quote_bch == 0 {
                    quote_bch = bch;
                } else if quote_bch == bch {
                    quote_bch = 0;
                }
            }
            _ => (),
        }
    }
    push_arg(&mut args, &line.as_bytes()[start..]);
    args
}

use std::path::PathBuf;
use std::env;
use file_assertions::{clear_dir, create_file};
use tt_core::journal::file::FileJournal;

#[derive(Clone, Default)]
pub struct TestPaths {
    test_dir: PathBuf,
    journal_file: PathBuf,
    config_file: PathBuf,
}

impl TestPaths {
    pub fn new(test_dir_name: &str) -> Self {
        TestPaths::default()
            .with_test_dir(&["target", test_dir_name])
            .with_journal_file("journal.txt")
            .with_config_file("tt-cli.toml")
    }

    pub fn with_test_dir(mut self, path: &[&str]) -> Self {
        self.test_dir = path.iter().collect::<PathBuf>();
        self
    }

    pub fn with_journal_file(mut self, file: &str) -> Self {
        self.journal_file = self.test_dir.join(file);
        self
    }

    pub fn with_config_file(mut self, file: &str) -> Self {
        self.config_file = self.test_dir.join(file);
        self
    }

    pub fn paths(&self) -> (&PathBuf, &PathBuf, &PathBuf) {
        (&self.journal_file, &self.config_file, &self.test_dir)
    }

    pub fn journal_file(&self) -> &PathBuf {
        &self.journal_file
    }

    pub fn init(&self) -> FileJournal {
        let (journal_file, config_file, test_dir) = self.paths();
        let config_content = format!("journal_file = {:?}", journal_file.as_os_str());

        clear_dir!(test_dir);
        create_file!(config_file, config_content);
        env::remove_var("TT_CLI_HOME");
        env::set_var("TT_CLI_CONFIG_FILE_NAME", config_file);
        FileJournal::new(journal_file)
    }
}