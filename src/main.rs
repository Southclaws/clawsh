use std::env::current_dir;

use clap::ArgMatches;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use starship::{context::Context, print::get_prompt};

mod parse;
mod procedure;

fn main() {
    let mut rl = Editor::<()>::new();
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    loop {
        let dir = current_dir().unwrap();
        let current_dir = Context::expand_tilde(dir.into());

        let prompt_text = get_prompt(Context::new_with_dir(ArgMatches::new(), current_dir));
        let readline = rl.readline(&prompt_text);

        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                println!("Line: {}", line);
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}
