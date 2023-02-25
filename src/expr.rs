use colored::*;

pub fn eval(expr: &str) {
    let expr_vec: Vec<char> = {
        let expr_string: String = expr.split_whitespace().collect();
        expr_string.chars().collect()
    };
    semantic_check(&expr_vec);
}

fn semantic_check(expr_vec: &Vec<char>) -> bool {
    println!("{:?}", expr_vec);
    for index in 0..expr_vec.len() {
        match expr_vec[index] {
            char if char.is_alphabetic() => {}

            '~' => {}

            '^' => {}

            'v' => {}

            '>' => {}

            '=' => {}

            '(' => match semantic_check(&expr_vec[index + 1..].to_owned()) {
                true => continue,
                false => error_print(expr_vec, index, "Missing ending parenthesis"),
            },

            ')' => {
                return true;
            }
            _ => error_print(expr_vec, index, "Invalid symbol"),
        }
    }
    false
}

fn error_print(expr_vec: &[char], index: usize, error: &str) {
    println!();
    for element in expr_vec.iter() {
        print!("{}", element.to_string().red());
    }
    println!();
    println!("{arrow:>index$}", arrow = "^".red(), index = index + 1);
    println!("{arrow:>index$}", arrow = "|".red(), index = index + 1);
    println!(
        "{error:>index$}",
        error = error.red(),
        index = index + error.len()
    );
    println!();
}
