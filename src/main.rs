use regex::Regex;
// #[warn(unused_imports)]
use std::{io::*, str::FromStr};

fn main() {

    // Declare all regex expression 

    let add_regex_expression = r"(\-?\d+)\s?\+\s?(\d+)";
    let subtract_regex_expression = r"(\-?\d+)\s?\-\s?(\d+)";
    let multiply_regex_expression = r"(\-?\d+)\s?\*\s?(\-?\d+)";
    let divide_regex_expression = r"(\-?\d+)\s?/\s?(\-?\d+)";

    // Datos del usuario 
    println!("Hello, welcome to the calculator");
    println!("Please enter your expression");

    // Take the expression from the user 
    let mut expression = "-10/-2".to_string();


    expression  = ApplyOperationInStringExpression(expression, divide_regex_expression.to_string(), "/");
    expression  = ApplyOperationInStringExpression(expression, multiply_regex_expression.to_string(), "*");
    expression  = ApplyOperationInStringExpression(expression, subtract_regex_expression.to_string(), "-");
    expression  = ApplyOperationInStringExpression(expression, add_regex_expression.to_string(), "+");

    println!("El resultado es: {:?}", expression);

}


/// This function is used to make the regex in the expression
#[warn(unused_variables, dead_code,)]
fn ApplyOperationInStringExpression(mut expression: String, string_regex:String, operation:&str) -> String {
    
    let regex_expression = Regex::new(&string_regex).unwrap();

    loop {
        let caps = regex_expression.captures(&expression);

        // Stop the loop if there are no more matches
        if caps.is_none() {
            break;
        }
        let caps = caps.unwrap();
        // Assign the values to the variables by the result
        let string_caps_statement = caps.get(0).unwrap().as_str();
        let left_value = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let right_value = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
        // Make the operation
        let result = match operation {
            "+" => left_value + right_value,
            "-" => left_value - right_value,
            "*" => left_value * right_value,
            "/" => left_value / right_value,
            _ => panic!("No se reconoce la operación"),
        };
        // replace the result number in the string expression
        expression = expression.replace(string_caps_statement, &result.to_string());
    }
    return expression;
}


