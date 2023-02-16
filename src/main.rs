use std::io;
use std::io::Write;
use colored::*;

mod expr;

fn main() {
    println!("{}","logic-calc (2023-2023) [github.com/ludihan/]".green().bold().italic());
    println!("{}","operations: [~][^][v][>][=]".green());
    println!("{}","type \"exit\" to close the program\n".green());

    loop {
        print!("{}","> ".bold());
        std::io::stdout().flush().unwrap();

        let input = {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            input
        };

        if input.trim().eq_ignore_ascii_case("exit") {
            break;
        }

        expr::eval(&input);
        //println!("{}", input);
    }
}
