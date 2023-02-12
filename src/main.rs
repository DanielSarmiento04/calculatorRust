use regex::Regex;
use std::io::*;


fn main() {

    // Datos del usuario 
    println!("Hello, welcome to the calculator");
    println!("Please enter your expression");

    // Take the expression from the user 
    // let mut expression =  String::new().to_owned();
    // stdin().read_line(&mut expression).unwrap();
    // expression = expression.trim().to_string();
    // // expression = expression.trim().to_string(); 
    // let mut string_confortable = String::new().to_owned();
    // string_confortable.push_str(r"(\-?\d+)\s?\");
    // string_confortable.push_str(&expression);
    // string_confortable.push_str(r"\s?(\d+)");
    // string_confortable.push_str(&expression);
    let expression = String::from("-12+-10");
    let result = make_regex_in_expression(expression, String::from("+"));

    // let result_sub = subtract_expression(expression);
    // let result_sum = sum_expression(result_sub);
    // let result = multiply_expression(expression);

    println!("Tu expresión es: {} tiene resultado ", result);
    // Validaciones de operaciones 
}


#[warn(unused_variables, dead_code,)]
fn multiply_expression(expression:String) -> String {
    let re_multiply : Regex = Regex::new(r"(\-?\d+)\s?\*\s?(\d+)").unwrap();
    // let response = make_regex_in_expression(expression, re_multiply);
    return "test".to_string();
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

/// This function is used to make the regex in the expression
#[warn(unused_variables, dead_code,)]
fn make_regex_in_expression(mut expression: String, operation:String) -> String {
    
    
    let mut custom_string = String::new().to_owned();
    custom_string.push_str(r"(\-?\d+)\s?\");
    custom_string.push_str(&operation);
    custom_string.push_str(r"\s?(\d+)");

    println!("La expresión regex es: {}", custom_string );
    let regex_expression : Regex = Regex::new(&custom_string).unwrap();
    loop {
        
        let caps = regex_expression.captures(&expression);

        // Stop the loop if there are no more matches
        if caps.is_none() {
            break;
        }

        let caps = caps.unwrap();

        let caps_expression = caps.get(0).unwrap().as_str();
        let left_value = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let right_value = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
        // let result = left_value + right_value;
        let mut result:i32 = 0;
        match operation.as_str() {
            "+" => {
                println!("La operación es suma");
                result = left_value + right_value;
            },
            "-" => {
                println!("La operación es resta");
                result = left_value - right_value;
            },
            "*" => {
                println!("La operación es multiplicación");
                result = left_value * right_value;
            },
            "/" => {
                println!("La operación es división");
                result = left_value / right_value;
            },
            _ => {
                panic!("No se reconoce la operación")
            }
        
            
        }

        expression = expression.replace(caps_expression, &result.to_string());
    }
    return expression;
}
