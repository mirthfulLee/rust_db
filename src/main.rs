pub mod executor;
pub mod sql_analyzer;
pub mod storage;

use executor::types::Executable;
use miette::GraphicalReportHandler;
use rustyline::error::ReadlineError;
use rustyline::history::FileHistory;
use rustyline::{Editor, Result};
use sql_analyzer::parser::Parse;
use sql_analyzer::types::SqlQuery;
use storage::*;

const HISTORY_FILE: &str = "./data/history.txt";

fn parse_and_execute(line: &str) {
    let path_root = StoreUtil::Csv(String::from(r"E:\git_commits\rust_db"));
    let parse_result = SqlQuery::parse_format_error(&line);
    match parse_result {
        Ok(query) => {
            let res = query.check_and_execute(path_root);
            match res {
                Ok(exec_res) => println!("{exec_res}"),
                Err(e) => {
                    let mut s = String::new();
                    GraphicalReportHandler::new()
                        .with_cause_chain()
                        .with_context_lines(10)
                        .render_report(&mut s, &e)
                        .unwrap();
                    println!("{s}");
                }
            }
        }
        Err(e) => {
            let mut s = String::new();
            GraphicalReportHandler::new()
                .render_report(&mut s, &e)
                .unwrap();
            println!("{s}");
        }
    }
}

fn main() -> Result<()> {
    let mut rl = Editor::<(), FileHistory>::new()?;
    if rl.load_history(HISTORY_FILE).is_err() {
        println!("No previous history.");
    }
    //path
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());
                parse_and_execute(line.as_str());
            }
            Err(ReadlineError::Interrupted) => {
                break;
                // CTRL-C so just skip
            }
            Err(ReadlineError::Eof) => {
                // CTRL-D so exit
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history(HISTORY_FILE)
}
