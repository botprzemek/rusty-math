use super::notations::postfix_expression;

mod structs;

pub fn postfix_expression(expression: &str) -> Stack<String> {
    let input: Vec<&str> = expression.split(" ").collect();
    let mut output: Stack<String> = Stack::new();
    let mut operators: Stack<char> = Stack::new();

    for text in input {
        match text.parse::<f32>() {
            Ok(number) => {
                output.push(number.to_string());
            }
            Err(_) => {
                if let Some(operator) = text.chars().collect::<Vec<char>>().get(0) {
                    println!("Operator: {}", operator);
                }
            }
        }
    }

    for _ in 0..operators.get().len() {
        if let Some(operator) = operators.pop() {
            output.push(operator.to_string());
        }
    }

    return output;
}