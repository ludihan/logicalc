use colored::*;

pub fn eval(expr: &str) {
    let expr_vec: Vec<char> = {
        let expr_string: String = expr.split_whitespace().collect();
        expr_string.chars().collect()
    };

    syntax_check(&expr_vec);
}

fn syntax_check(expr_vec: &Vec<char>) {
    println!("{:?}", expr_vec);
    let mut opened_parenthesis = 0;

    if expr_vec[expr_vec.len() - 1] == '~'
        || expr_vec[expr_vec.len() - 1] == '^'
        || expr_vec[expr_vec.len() - 1] == 'v'
        || expr_vec[expr_vec.len() - 1] == '>'
        || expr_vec[expr_vec.len() - 1] == '='
        || expr_vec[expr_vec.len() - 1] == '('
    {
        error_print(expr_vec, expr_vec.len(), "syntax error");
    }

    for index in 0..expr_vec.len() {
        match expr_vec[index] {
            char if char.is_ascii_uppercase() => match expr_vec[index - 1] {
                '~' | '^' | 'v' | '>' | '=' | '(' => continue,
                ')' if opened_parenthesis != 0 => continue,
                _ => error_print(expr_vec, index, "syntax error"),
            },

            '~' => match expr_vec[index - 1] {
                '~' | '^' | 'v' | '>' | '=' => continue,
                _ => error_print(expr_vec, index, "syntax error"),
            },

            '^' => match expr_vec[index - 1] {
                previous_char if previous_char.is_ascii_uppercase() => continue,
                _ => error_print(expr_vec, index, "syntax error"),
            },

            'v' => match expr_vec[index - 1] {
                previous_char if previous_char.is_ascii_uppercase() => continue,
                _ => error_print(expr_vec, index, "syntax error"),
            },

            '>' => match expr_vec[index - 1] {
                previous_char if previous_char.is_ascii_uppercase() => continue,
                _ => error_print(expr_vec, index, "syntax error"),
            },

            '=' => match expr_vec[index - 1] {
                previous_char if previous_char.is_ascii_uppercase() => continue,
                _ => error_print(expr_vec, index, "syntax error"),
            },

            '(' => match expr_vec[index - 1] {
                '~' | '^' | 'v' | '>' | '=' => {
                    opened_parenthesis += 1;
                    continue;
                }
                _ => error_print(expr_vec, index, "syntax error"),
            },

            ')' => match expr_vec[index - 1] {
                previous_char if previous_char.is_ascii_uppercase() || opened_parenthesis != 0 => {
                    opened_parenthesis -= 1;
                    continue;
                }
                _ => error_print(expr_vec, index, "syntax error"),
            },

            _ => {
                error_print(expr_vec, index, "illegal symbol");
                break;
            }
        }
    }
    expr_slicer(expr_vec);
}

fn expr_slicer(expr_vec: &Vec<char>) {
    let mut vec: Vec<Vec<char>> = Vec::new();
}

fn expr_evaluator(expr_vec: &Vec<char>) {}

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
