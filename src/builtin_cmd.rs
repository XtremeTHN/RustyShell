use crossterm::{execute, terminal};
use crossterm::cursor::MoveTo;
use std::{env, process::{Command, Stdio}, path::PathBuf};
use std::io::{self, Write, BufRead};
use term_size::dimensions;
use colored::*;

pub fn clear_screen() -> io::Result<()> {
    let mut stdout = io::stdout();
    execute!(stdout, terminal::Clear(terminal::ClearType::All), MoveTo(0, 0))?;
    stdout.flush()?;
    Ok(())
}

pub fn list_cmd(work_dir: String) -> Result<(), String> {
    let work_dir_convertion = PathBuf::from(&work_dir);
    let mut colored_vector: Vec<String> = vec![];
    //if let Ok(iterator) = work_dir_convertion.read_dir() {
    match work_dir_convertion.read_dir() {
        Ok(iterator) => {
            for x in iterator {
                if x.as_ref().unwrap().path().is_dir() {
                    let file = x.unwrap().path().clone();
                    colored_vector.push(file.file_name().unwrap().to_str().unwrap().blue().to_string());
                    
                } else if x.as_ref().unwrap().path().is_symlink() {
                    let file = x.unwrap().path().clone();
                    colored_vector.push(file.file_name().unwrap().to_str().unwrap().bright_yellow().to_string());
                } else {
                    let file = x.unwrap().path().clone();
                    colored_vector.push(file.file_name().unwrap().to_str().unwrap().yellow().to_string());
                }
            }
            columnize_text(&colored_vector);
            Ok(())
        }
        Err(err) => Err(format!("Cannot read the directory. Error: {}", err)),
    }
}

pub fn columnize_text(items: &[String]) {
    if let Some((width, _)) = dimensions() {
        let longest_item_len = items.iter().map(|item| item.len()).max().unwrap_or(0);
        let num_columns = width / (longest_item_len + 2).max(1);
        let num_rows = (items.len() as f64 / num_columns as f64).ceil() as usize;

        for row in 0..num_rows {
            for column in 0..num_columns {
                let index = row + column * num_rows;
                if let Some(item) = items.get(index) {
                    print!("{:<width$}", item, width = longest_item_len + 2);
                }
            }
            println!();
        }
    } else {
        for item in items {
            println!("{}", item);
        }
    }
}

pub fn run_external_command(executable_name: &str, args: Option<Vec<String>>) -> () {
    if let Some(paths) = env::var_os("PATH") {
        for path in env::split_paths(&paths) {
            let executable_path = path.join(executable_name);
            if executable_path.is_file() {
                if let Some(mut prog_args) = args.clone() {
                    prog_args.drain(0..1);
                    let mut output = Command::new(executable_path)
                    .args(prog_args)
                    .stdout(Stdio::piped())
                    .spawn();
                    match output {
                        Ok(mut out) => {
                            if let Some(stdout) = out.stdout.take() {
                                let reader = io::BufReader::new(stdout);
                                for line in reader.lines() {
                                    if let Ok(line) = line {
                                        println!("{}", line);
                                    }
                                }
                            }
                        }
                        Err(err) => {
                            println!("Couldn't retrieve the command output");
                        }
                    }
                }
            }
        }
    }
}