//use std::io;
//use std::io::Write;
use colored::*;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};
mod logic;
fn main() -> Result<()> {
    println!(
        "{}",
        "logicalc (2023-2023) [github.com/ludihan/logicalc]"
            .green()
            .bold()
            .italic()
    );
    println!("{}", "operations: [~][^][v][>][=][(][)]".green());
    println!("{}", "use UPPERCASE ascii characters for variables".green());
    println!("{}", "type \"exit\" to close the program\n".green());

    let mut rl = DefaultEditor::new()?;
    #[cfg(feature = "with-file-history")]
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                if line.trim().eq_ignore_ascii_case("exit") {
                    break;
                }
                logic::from(&line);
                //println!("Line: {}", line);
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C pressed. bye");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D pressed. bye");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    #[cfg(feature = "with-file-history")]
    rl.save_history("history.txt");
    Ok(())

    /*
    loop {
        print!("{}", "> ".bold());
        std::io::stdout().flush().unwrap();

        let input = {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            input
        };

        if input.trim().eq_ignore_ascii_case("exit") {
            break;
        }

        logic::from(&input);
    }
    */
}
