//mod plugins; 
mod logger; mod builtin_cmd;

//use plugins::load_python_plugin_init_files;
use std::{thread, path::PathBuf, env::{set_current_dir, current_dir}};
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor};
use directories::ProjectDirs;
use shellwords::split;
use logger::init_log;
use builtin_cmd::*;
use log::{info};
use colored::*;



fn main() {
    let binding = ProjectDirs::from("", "", "RustyShell").unwrap();
    let data_folder = binding.data_dir();
    init_log(PathBuf::from(data_folder));
    info!("Log init successfull");
    info!("Python scripts init in progress...");
    /*let thread = thread::spawn(|| {
        load_python_plugin_init_files();
    });*/

    let mut rl = DefaultEditor::new().unwrap();
    #[cfg(feature = "with-file-history")]
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    
    let mut prompt = format!("{} >> ", current_dir().unwrap().to_string_lossy());
    loop {
        let readline = rl.readline(&prompt);
        match readline {
            Ok(line) => {
                if let Err(err) = rl.add_history_entry(line.as_str()) {
                    println!("{}: History cannot be saved", "Error".red());
                    println!("{}", err);
                }
                let shell_cmd = split(&line);
                if let Err(err) = shell_cmd {
                    println!("Debug: Cannot parse command: {}", err);
                    continue;
                }
                let shell_cmd = shell_cmd.unwrap();
                
                if let Some(unknown_cmd) = shell_cmd.get(0) {
                    match unknown_cmd.as_str() {
                        "ls" => {
                            if shell_cmd.get(1).is_some() {
                                if let Err(err) = list_cmd(shell_cmd[1].clone()) {
                                    println!("ls: {}", err);
                                };
                            } else {
                                if let Err(err) = list_cmd(".".to_string()) {
                                    println!("ls: {}", err);
                                };
                            }
                        },
                        "cd" => {
                            if shell_cmd.get(1).is_some() {
                                if let Err(err) = set_current_dir(shell_cmd[1].clone()) {
                                    println!("cd: {}", err);
                                } else {
                                    let binded = current_dir().unwrap();
                                    prompt = format!("{} >> ", binded.to_string_lossy());
                                }
                            }
                        }

                        "echo" => {
                            println!("{}", shell_cmd.iter().skip(1).cloned().collect::<Vec<String>>().join(" "));
                        }

                        "clear" => {
                            if let Err(_) = clear_screen() {
                                println!("clear: Error while trying to clear the terminal")
                            };
                        }

                        "exit" => break,
                        &_ => {
                            let mut sh_cmd = shell_cmd[0].to_string();
                            sh_cmd.push_str(".exe");
                            run_external_command(&sh_cmd, Some(shell_cmd.clone()))
                        }
                    }
                }
            },
            Err(ReadlineError::Interrupted) => {
                break
            },
            Err(ReadlineError::Eof) => {
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
    //thread.join();
}