mod interpreter;
mod lexer;
mod parser;
mod feather;

use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

fn main() -> rustyline::Result<()> {
    println!("Pelikan Interpreter (pelin) v0.1.0");
    println!("Type 'exit' to quit the REPL");

    let mut rl = DefaultEditor::new()?;

    loop {
        let readline = rl.readline("pelin> ");
        match readline {
            Ok(line) => {
                if line.trim() == "exit" {
                    break;
                }
                rl.add_history_entry(line.as_str())?;

                println!("Echo: {}", line);
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    Ok(())
}