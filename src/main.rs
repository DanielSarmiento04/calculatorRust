use std::io::stdin;
use regex::Regex;

/// This function is used to make the regex in the expression
#[allow(non_snake_case)]
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

#[allow(dead_code)]
fn examples () {
    let add_regex_expression = r"(\-?\d+)\s?\+\s?(\d+)";
    let subtract_regex_expression = r"(\-?\d+)\s?\-\s?(\d+)";
    let multiply_regex_expression = r"(\-?\d+)\s?\*\s?(\-?\d+)";
    let divide_regex_expression = r"(\-?\d+)\s?/\s?(\-?\d+)";

    // Declare the expressions to test
    let examples = vec![
            "1 + 2 * 3 - 4 / 2",
            " -23 -32 +12 *32",
            " 89 *-32-3+53"
        ];

    examples.iter().for_each(|example| {
        let expression = example.trim().to_string();
        let mut result  = ApplyOperationInStringExpression(expression.clone(), divide_regex_expression.to_string(), "/");
        result  = ApplyOperationInStringExpression(result, multiply_regex_expression.to_string(), "*");
        result  = ApplyOperationInStringExpression(result, subtract_regex_expression.to_string(), "-");
        result  = ApplyOperationInStringExpression(result, add_regex_expression.to_string(), "+");
        println!("El resultado de {} es:   {}", expression, result);
    });
}


fn main() {
    // examples();
    // Declare all regex expression 
    let add_regex_expression = r"(\-?\d+)\s?\+\s?(\d+)";
    let subtract_regex_expression = r"(\-?\d+)\s?\-\s?(\d+)";
    let multiply_regex_expression = r"(\-?\d+)\s?\*\s?(\-?\d+)";
    let divide_regex_expression = r"(\-?\d+)\s?/\s?(\-?\d+)";

    // Datos del usuario 
    println!("Hello, welcome to the calculator");
    println!("Please enter your expression");

    // Take the expression from the user 
    let mut expression = String::new().to_owned();

    stdin().read_line(&mut expression).expect("Failed to read line");

    expression = expression.trim().to_string();
    
    expression  = ApplyOperationInStringExpression(expression, divide_regex_expression.to_string(), "/");
    expression  = ApplyOperationInStringExpression(expression, multiply_regex_expression.to_string(), "*");
    expression  = ApplyOperationInStringExpression(expression, subtract_regex_expression.to_string(), "-");
    expression  = ApplyOperationInStringExpression(expression, add_regex_expression.to_string(), "+");

    println!("El resultado es: {:?}", expression);

}



