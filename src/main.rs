mod sql_analyzer;
mod executor;
mod storage;

use rustyline::error::ReadlineError;
use rustyline::history::FileHistory;
use rustyline::{Editor, Result};

const HISTORY_FILE: &str = "./data/history.txt";

fn main() -> Result<()> {
    let mut rl = Editor::<(), FileHistory>::new()?;
    if rl.load_history(HISTORY_FILE).is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());
                println!("Line: {}", line);
            },
            Err(ReadlineError::Interrupted) => {
                // CTRL-C so just skip
            },
            Err(ReadlineError::Eof) => {
                // CTRL-D so exit
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    rl.save_history(HISTORY_FILE)
}