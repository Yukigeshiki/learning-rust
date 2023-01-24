use custom_db as db;
use std::io;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello! This is db");
    println!("Feel free to type in commands");

    let stdin = io::stdin();
    let stdout = io::stdout();

    db::repl::start(&mut stdin.lock(), &mut stdout.lock())
        .unwrap_or_else(|e| panic!("An error occurred: {}", e));

    Ok(())
}