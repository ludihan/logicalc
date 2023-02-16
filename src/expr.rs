use colored::*;

pub fn eval(expr: &str) {
    syntax_check(expr);
}

fn syntax_check(expr: &str) {
    //let expr_string: String = expr.split_whitespace().collect();
    //let expr_vec: Vec<char> = expr_string.chars().collect();
    
    let expr_vec: Vec<char> = {
        let expr_string: String = expr.split_whitespace().collect();
        expr_string.chars().collect()
    };

    //let previous_element: char = ' ';

    for index in 0..expr_vec.len() {
        if expr_vec[index].is_alphabetic() {
            continue;
        } else if expr_vec[index] == '(' || expr_vec[index] == ')' {
            continue;
        }else if 
            expr_vec[index] == '~' || 
            expr_vec[index] == '^' ||
            expr_vec[index] == 'v' ||
            expr_vec[index] == '>' ||
            expr_vec[index] == '=' 
        {
            continue;
        } else {
            //println
            error_print(&expr_vec, index, "Invalid symbol");
        }
    }
}

fn error_print(expr_vec: &Vec<char>, index: usize, error: &str) {
    println!("");
    for element in expr_vec.iter() {
        print!("{}",element.to_string().red());
    }
    println!("");
    println!("{arrow:>index$}", arrow = "^".red(), index = index + 1);
    println!("{arrow:>index$}", arrow = "|".red(), index = index + 1);
    println!("{error:>index$}", error = error.red(), index = index + error.len());
}
