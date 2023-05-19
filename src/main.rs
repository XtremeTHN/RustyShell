mod plugins; mod logger;

use plugins::load_python_plugin_init_files;
use rustyline::{DefaultEditor, Result};
use rustyline::error::ReadlineError;
use log::{info, warn, error, debug};
use std::{thread, path::PathBuf};
use directories::ProjectDirs;
use logger::init_log;

fn main() {
    let binding = ProjectDirs::from("", "", "RustyShell").unwrap();
    let data_folder = binding.data_dir();
    init_log(PathBuf::from(data_folder));
    info!("Log init successfull");
    info!("Python scripts init in progress...");
    let thread = thread::spawn(|| {
        load_python_plugin_init_files();
    });

    let mut rl = DefaultEditor::new().unwrap();
    #[cfg(feature = "with-file-history")]
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                println!("Line: {}", line);
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    #[cfg(feature = "with-file-history")]
    rl.save_history("history.txt");
}