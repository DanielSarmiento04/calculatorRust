use regex::Regex;
use std::io::*;


fn main() {

    // Datos del usuario 
    println!("Hello, welcome to the calculator");
    println!("Please enter your expression");

    // Take the expression from the user 
    let mut expression =  String::new();
    stdin().read_line(&mut expression).unwrap();
    expression = expression.trim().to_string(); 

    let result_sub = subtract_expression(expression);
    let result_sum = sum_expression(result_sub);

    println!("Tu expresión es: {} tiene resultado ", result_sum);

    // Validaciones de operaciones 
}

#[warn(unused_variables, dead_code,)]
fn sum_expression(mut expression: String) -> String {
    // Validaciones de operaciones
    let re_add : Regex = Regex::new(r"(\-?\d+)\s?\+\s?(\d+)").unwrap();
        
    loop {

        let caps = re_add.captures(&expression);

        // Stop the loop if there are no more matches
        if caps.is_none() {
            break;
        }

        let caps = caps.unwrap();

        let caps_expression = caps.get(0).unwrap().as_str();
        let left_value = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let right_value = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
        let result = left_value + right_value;

        expression = expression.replace(caps_expression, &result.to_string());
    }
    return expression;
}

#[warn(unused_variables, dead_code,)]
fn subtract_expression(mut expression: String) -> String {
    // Validaciones de operaciones
    let re_subs : Regex = Regex::new(r"(\-?\d+)\s?\-\s?(\d+)").unwrap();
    loop {
        
        let caps = re_subs.captures(&expression);

        // Stop the loop if there are no more matches
        if caps.is_none() {
            break;
        }

        let caps = caps.unwrap();

        let caps_expression = caps.get(0).unwrap().as_str();
        let left_value = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let right_value = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
        let result = left_value - right_value;

        expression = expression.replace(caps_expression, &result.to_string());
    }
    return expression;
}


