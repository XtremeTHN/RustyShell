use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};
use shell_words::split;

fn main() -> Result<()> {
    // `()` can be used when no completer is required
    let mut rl = DefaultEditor::new()?;
    #[cfg(feature = "with-file-history")]
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    let exit_cmd = String::from("exit");

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                
                if let Ok(args) = split(line.as_str()) {
                    if args.get(0).unwrap() == "install" {
                        if args.get(1).unwrap() == "command" {
                            println!("Installing {}", args.get(2).unwrap())
                        }
                    }
                }
                
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
    Ok(())
}