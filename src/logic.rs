use colored::*;

pub fn from(expr: &str) {
    let logic_vec: Vec<char> = {
        let logic_string: String = expr.split_whitespace().collect();
        logic_string.chars().collect()
    };

    syntax_check(logic_vec);
}

fn syntax_check(logic_vec: Vec<char>) {
    let mut open_parenthesis = 0;

    // first element
    match logic_vec[0] {
        '(' => open_parenthesis += 1,
        '~' => (),
        char if char.is_ascii_uppercase() => (),
        _ => {
            error_print(&logic_vec, 0, "syntax error");
            return;
        }
    }

    //last element
    match logic_vec[logic_vec.len() - 1] {
        ')' => (),
        char if char.is_ascii_uppercase() => (),
        _ => {
            error_print(&logic_vec, logic_vec.len() - 1, "syntax error");
            return;
        }
    }

    // all other elements
    // ~ ^ v > = ) ( A
    for index in 0..logic_vec.len() - 1 {
        match logic_vec[index] {
            char if char.is_ascii_uppercase() => match logic_vec[index + 1] {
                '^' | 'v' | '>' | '=' => continue,
                ')' => {
                    open_parenthesis -= 1;
                    continue;
                }
                _ => {
                    error_print(&logic_vec, index, "syntax error");
                    return;
                }
            },

            '~' => match logic_vec[index + 1] {
                next_char if next_char.is_ascii_uppercase() => continue,
                '(' => {
                    open_parenthesis += 1;
                    continue;
                }
                ')' => {
                    open_parenthesis -= 1;
                    continue;
                }
                '~' => continue,
                _ => {
                    error_print(&logic_vec, index, "syntax error");
                    return;
                }
            },

            '^' => match logic_vec[index + 1] {
                next_char if next_char.is_ascii_uppercase() => continue,
                '(' => {
                    open_parenthesis += 1;
                    continue;
                }
                ')' => {
                    open_parenthesis -= 1;
                    continue;
                }
                '~' => continue,
                _ => {
                    error_print(&logic_vec, index, "syntax error");
                    return;
                }
            },

            'v' => match logic_vec[index + 1] {
                next_char if next_char.is_ascii_uppercase() => continue,
                '(' => {
                    open_parenthesis += 1;
                    continue;
                }
                ')' => {
                    open_parenthesis -= 1;
                    continue;
                }
                '~' => continue,
                _ => {
                    error_print(&logic_vec, index, "syntax error");
                    return;
                }
            },

            '>' => match logic_vec[index + 1] {
                next_char if next_char.is_ascii_uppercase() => continue,
                '(' => {
                    open_parenthesis += 1;
                    continue;
                }
                ')' => {
                    open_parenthesis -= 1;
                    continue;
                }
                '~' => continue,
                _ => {
                    error_print(&logic_vec, index, "syntax error");
                    return;
                }
            },

            '=' => match logic_vec[index + 1] {
                next_char if next_char.is_ascii_uppercase() => continue,
                '(' => {
                    open_parenthesis += 1;
                    continue;
                }
                ')' => {
                    open_parenthesis -= 1;
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
                    open_parenthesis += 1;
                    continue;
                }
                _ => {
                    error_print(&logic_vec, index, "syntax error");
                    return;
                }
            },

            ')' => match logic_vec[index + 1] {
                '^' | 'v' | '>' | '=' => {
                    continue;
                }
                ')' => {
                    open_parenthesis -= 1;
                    continue;
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

    if open_parenthesis > 0 {
        error_print(&logic_vec, logic_vec.len() - 1, "missing parenthesis");
        return;
    }

    logic_slicer(logic_vec);
}

// ((AvAvA)vA)
fn logic_slicer(logic_vec: Vec<char>) {
    let mut vec_of_vecs: Vec<Vec<char>> = vec![];
    let mut open_parenthesis = 0;
    for index in 0..logic_vec.len() {
        match logic_vec[index] {
            '(' => {
                open_parenthesis += 1;
                for inner_index in index..logic_vec.len() {
                    match logic_vec[inner_index] {
                        '(' => open_parenthesis += 1,
                        ')' => {
                            open_parenthesis -= 1;
                            if open_parenthesis == 1 {
                                vec_of_vecs.push(logic_vec[index..=inner_index].to_vec());
                                open_parenthesis -= 1;
                                break;
                            }
                        }
                        _ => (),
                    }
                }
            }
            ')' => open_parenthesis -= 1,
            _ => (),
        }
    }

    vec_of_vecs.push(logic_vec);

    logic_eval(vec_of_vecs);
}

fn logic_eval(vec_of_vecs: Vec<Vec<char>>) {
    println!("{:#?}", vec_of_vecs);
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
