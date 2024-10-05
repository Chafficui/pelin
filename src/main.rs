use std::env;
use std::fs;
use std::process;
use pelin::lexer::Lexer;
use pelin::parser::Parser;
use pelin::interpreter::Interpreter;
use pelin::feather::FeatherManager;
use std::rc::Rc;
use std::cell::RefCell;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1).map(String::as_str) {
        Some("--version") => {
            println!("pelin version {}", VERSION);
            return;
        }
        Some(filename) => {
            if let Err(err) = run_file(filename) {
                eprintln!("Error: {}", err);
                process::exit(1);
            }
        }
        None => {
            print_usage();
            process::exit(1);
        }
    }
}

fn print_usage() {
    println!("Usage: pelin <file.pl>");
    println!("       pelin --version");
    println!("\nRuns Pelikan programs or displays the version of pelin.");
}

fn run_file(filename: &str) -> Result<(), String> {
    if !filename.ends_with(".pl") {
        return Err(format!("Invalid file extension. Expected a .pl file, got: {}", filename));
    }

    let content = fs::read_to_string(filename)
        .map_err(|e| format!("Error reading file '{}': {}", filename, e))?;

    let mut lexer = Lexer::new(&content);
    let tokens = lexer.tokenize()?;

    let mut parser = Parser::new(tokens);
    let expressions = parser.parse()?;

    let project_root = std::env::current_dir().unwrap();
    let feather_manager = Rc::new(RefCell::new(FeatherManager::new(project_root)));
    let interpreter = Interpreter::new(Rc::clone(&feather_manager));

    interpreter.interpret_program(&expressions)?;

    Ok(())
}