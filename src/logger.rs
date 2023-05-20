use simplelog::*;
use colored::*;
use chrono::{Local, Datelike, Timelike};
use std::fs::File;
use std::path::PathBuf;

pub fn init_log(mut data_folder: PathBuf) -> () {
    // Crea el nombre del archivo
    let now = Local::now();
    let file_name = format!("{:04}-{:02}-{:02}-{:02}-{:02}-{:02}-file.log", now.year(), now.month(), now.day(), now.hour(), now.minute(), now.second());

    if let Err(err) = std::fs::create_dir_all(data_folder.clone()) {
        println!("{}: Error while trying to create unexisting directories: {}", "Error".red(), err);
        return;
    };
    // Creando archivo
    data_folder.push(file_name);

    let log_file = File::create(data_folder);

    if let Ok(log_file) = log_file {
        // Configura el logger para escribir en el archivo
        if let Err(err) = CombinedLogger::init(
            vec![
                WriteLogger::new(LevelFilter::Info, Config::default(), log_file),
            ]
        ) {
            println!("{}: Error while trying to init the logger {}", "Error".red(), err);
            return;
        }
    };


}