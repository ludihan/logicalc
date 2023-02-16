pub fn eval(expr: &str) {
    syntax_check(expr);
}

fn syntax_check(expr: &str) {

    let expr_string: String = expr.split_whitespace().collect();
    let expr_vec: Vec<char> = expr_string.chars().collect();
    
    let previous_element: char = ' ';
    for elements in expr_vec.iter() {
        
    }
    println!("{:?}", expr_vec);
}