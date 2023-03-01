use colored::*;
use ascii_table::{AsciiTable, Align};

pub fn from(expr: &str) {
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

    evaluator(logic_vecs)
}

fn evaluator(logic_vecs: Vec<String>) {
    let mut truth_table: Vec<Vec<String>> = vec![vec!["s".to_string(),"s".to_string(),
    "s".to_string(),]];

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

    

    // Variable substitution
    let mut copy_logic_vecs = logic_vecs.clone();
    

    // Actual evaluation
    // TODO

    let mut ascii_table = AsciiTable::default();
    ascii_table.set_max_width(300);

    let mut set_index_header = 0;
    for index in 0..variables.len() {
        ascii_table.column(index).set_header(&variables[set_index_header]).set_align(Align::Center);
        set_index_header += 1;
    }

    for index in 0..logic_vecs.len() - 2 {
        ascii_table.column(set_index_header+index).set_header(&logic_vecs[set_index_header+index]).set_align(Align::Center);
    }
    ascii_table.print(truth_table);

    println!("{:#?}", variables);
    println!("{:#?}", logic_vecs);
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
