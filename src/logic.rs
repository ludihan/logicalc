use colored::*;
use ascii_table::{AsciiTable, Align};
use std::io::{stdin, Write};
use std::vec;

pub fn from(expr: &str) {
    // conversion from string slice to vec of chars
    
    let logic_vec: Vec<char> = {
        let logic_string: String = expr.split_whitespace().collect();
        logic_string.chars().collect()
    };

    syntax_check(logic_vec);
}

fn syntax_check(logic_vec: Vec<char>) {
    let mut open_parenthesis = false;

    // first element
    match logic_vec[0] {
        var if var.is_ascii_uppercase() => {
            for element in logic_vec.iter() {
                match element {
                    ')' => {
                        error_print(&logic_vec, 0, "missing matching parenthesis");
                        return;
                    },
                    _ => continue,
                }
            }
        }
        '(' => open_parenthesis = true,
        '~' => (),
        _ => {
            error_print(&logic_vec, 0, "syntax error");
            return;
        }
    }

    // other elements
    for index in 0..logic_vec.len() - 1 {
        match logic_vec[index] {
            var if var.is_ascii_uppercase() => match logic_vec[index + 1] {
                '^' | 'v' | '>' | '=' => continue,
                ')' => open_parenthesis = false,
                _ => {
                    error_print(&logic_vec, index, "syntax error");
                    return;
                }
            },

            '~' => match logic_vec[index + 1] {
                next_char if next_char.is_ascii_uppercase() => continue,
                '(' => {
                    open_parenthesis = true;
                    continue;
                }
                '~' | '^' | 'v' | '>' | '=' => continue,
                _ => {
                    error_print(&logic_vec, index, "syntax error");
                    return;
                }
            },

            '^' | 'v' | '>' | '=' => match logic_vec[index + 1] {
                next_char if next_char.is_ascii_uppercase() => continue,
                '(' => {
                    open_parenthesis = true;
                    continue;
                }
                ')' => {
                    open_parenthesis = false;
                    continue;
                }
                '~' => continue,
                _ => {
                    error_print(&logic_vec, index, "syntax error");
                    return;
                }
            },

            '(' => match logic_vec[index + 1] {
                next_char if next_char.is_ascii_uppercase() => {
                    continue;
                }
                '~' | '^' | 'v' | '>' | '=' => {
                    continue;
                }
                '(' => {
                    error_print(&logic_vec, index, "cant use nested parenthesis");
                    return;
                }
                _ => {
                    error_print(&logic_vec, index, "syntax error");
                    return;
                }
            },

            ')' => match logic_vec[index + 1] {
                next_char if next_char.is_ascii_uppercase() => {
                    continue;
                }
                '~' | '^' | 'v' | '>' | '=' => {
                    continue;
                }
                ')' => {
                    error_print(&logic_vec, index, "cant use nested parenthesis");
                    return;
                }
                _ => {
                    error_print(&logic_vec, index, "syntax error");
                    return;
                }
            },

            _ => {
                error_print(&logic_vec, index, "illegal symbol");
                return;
            }
        }
    }

    //last element
    match logic_vec[logic_vec.len() - 1] {
        char if char.is_ascii_uppercase() => (),
        ')' => {
            if open_parenthesis == false {
                ();
            } else {
                error_print(&logic_vec, logic_vec.len() - 1, "syntax error");
            }
        }
        _ => {
            error_print(&logic_vec, logic_vec.len() - 1, "syntax error");
            return;
        }
    }

    if open_parenthesis == true {
        println!("{open_parenthesis}");
        error_print(&logic_vec, logic_vec.len() - 1, "syntax error parenthesis");
        return;
    }

    logic_slicer(logic_vec);
}

fn logic_slicer(logic_vec: Vec<char>) {
    let mut logic_vecs: Vec<String> = Vec::new();
    let mut open_parenthesis_index = 0;
    let mut close_parenthesis_index = 0;

    // slice parenthesis
    for index in 0..logic_vec.len() {
        match logic_vec[index] {
            '(' => {
                open_parenthesis_index = index;
            }
            ')' => {
                close_parenthesis_index = index;
                logic_vecs.push(
                    logic_vec[open_parenthesis_index..=close_parenthesis_index]
                        .into_iter()
                        .collect(),
                );
            }
            _ => continue,
        }
    }

    logic_vecs.push(logic_vec.into_iter().collect());

    // Count variables
    let mut variables: Vec<String> = Vec::new();
    for string_index in 0..logic_vecs.len() {
        for char_element in logic_vecs[string_index].chars() {
            match char_element {
                var if var.is_ascii_uppercase() => {
                    if !variables.contains(&var.to_string()) {
                        variables.push(var.to_string());
                    } else {
                        continue;
                    }
                }
                _ => continue,
            }
        }
    }

    table_print(logic_vecs, variables)
}

fn table_print(logic_vecs: Vec<String>, variables: Vec<String>) {
    let mut ascii_table = AsciiTable::default();
    ascii_table.set_max_width(200);

    for (index, element) in logic_vecs.iter().enumerate() {
        ascii_table.column(index).set_header(element).set_align(Align::Center);
    }

    //let data = vec![&logic_vecs];
    //ascii_table.print(data);

    // Variable substitution
    let truth_table = alternator(logic_vecs, variables);
    ascii_table.print(truth_table);

}

fn alternator(mut logic_vecs: Vec<String>, variables: Vec<String>) -> Vec<Vec<String>> {
    let mut truth_table: Vec<Vec<String>> = Vec::new();
    let mut variables_value: Vec<String> = Vec::new();
    let mut index = 0;

    loop {
        let mut value: String = String::new();
        print!("Choose the value for variable {}: ", &variables[index]);
        std::io::stdout().flush().unwrap();
        stdin().read_line(&mut value).unwrap();

        match value.trim() {
            "1" | "0" => (),
            _ => {
                println!("only 1 or 0 allowed!");
                continue;
            }
        }

        index += 1;

        variables_value.push(value.trim().to_string());

        

        if variables.len() == variables_value.len() {
            break;
        }
    }
    
    replace_chars(&mut logic_vecs, &variables, &variables_value);

    for mut elements in &mut logic_vecs {
        //println!("{elements}");
        truth_table.push(vec!(evaluator(&mut elements)));
    }

    println!("logic :{:#?}", logic_vecs);

    truth_table

}

fn replace_chars(vec1: &mut Vec<String>, vec2: &Vec<String>, vec3: &Vec<String>) {
    vec1.iter_mut().for_each(|s| {
        let new_chars = s.chars().enumerate().map(|(j, c)| {
            match vec2.iter().position(|s| s == &c.to_string()) {
                Some(index) => vec3.get(index).unwrap_or(&"".to_string()).to_string(),
                None => c.to_string(),
            }
        }).collect::<Vec<String>>();
        s.clear();
        s.push_str(&new_chars.concat());
    });
}

fn evaluator(logic_vec: &str) -> String {
    let mut stack = Vec::new();
    let mut result = String::new();
    let mut index = 0;

    stack.push(logic_vec.chars().nth(0).unwrap().to_string());
    while index < logic_vec.len() {
        let c = logic_vec.chars().nth(index).unwrap();

        match c {
            '(' => {
                let mut subexpr = String::new();
                let mut count = 1;
                index += 1;
                let mut closing_parenthesis_index = 0;
                for inner_index in index.. {
                    match logic_vec.as_bytes()[inner_index].to_string().as_str() {
                        ")" => closing_parenthesis_index = inner_index,
                        _ => (),
                    }

                    let value = evaluator(&logic_vec.clone()[index..inner_index]);
                    stack.push(value);
                }

                let value = evaluator(&logic_vec[(index + 1)..]);
                stack.push(value);
            }
            '~' => {
                let operand = stack.pop().unwrap();
                let value = if operand == "0" { "1" } else { "0" };
                stack.push(value.to_string());
                index += 1;
            }
            '^' => {
                let operand2 = stack.pop().unwrap();
                let operand1 = stack.pop().unwrap();
                let value = if operand1 == "1" && operand2 == "1" { "1" } else { "0" };
                stack.push(value.to_string());
                index += 1;
            }
            'v' => {
                let operand2 = stack.pop().unwrap();
                let operand1 = stack.pop().unwrap();
                let value = if operand1 == "1" || operand2 == "1" { "1" } else { "0" };
                stack.push(value.to_string());
                index += 1;
            }
            '>' => {
                let operand2 = stack.pop().unwrap();
                let operand1 = stack.pop().unwrap();
                let value = if operand1 == "0" || operand2 == "1" { "1" } else { "0" };
                stack.push(value.to_string());
                index += 1;
            }
            '=' => {
                let operand2 = stack.pop().unwrap();
                let operand1 = stack.pop().unwrap();
                let value = if operand1 == operand2 { "1" } else { "0" };
                stack.push(value.to_string());
                index += 1;
            }
            '0' | '1' => {
                stack.push(c.to_string());
                index += 1;
            }
            _ => {
                panic!("Invalid character '{}' at index {}", c, index);
            }
        }
    }

    if let Some(value) = stack.pop() {
        result = value;
    }

    result
}



fn error_print(logic_vec: &[char], index: usize, error: &str) {
    println!();
    for element in logic_vec.iter() {
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
