use crate::parser::parse_expression;
use crate::structs::Stack;

fn get_precedence(operator: String) -> i8 {
    match operator.as_str() {
        "+" | "-" => 1,
        "*" | "/" | "%" => 2,
        "^" => 3,
        _ => 0,
    }
}

fn handle_operator(operator: String, output: &mut Stack<String>, operators: &mut Stack<String>) {
    match operator.as_str() {
        "+" | "-" | "*" | "/" | "%" | "^" => {
            while let Some(top) = operators.top() {
                if get_precedence(top.to_string()) < get_precedence(operator.to_string()) {
                    break;
                }

                output.push(top.to_string());
                operators.pop();
            }

            operators.push(operator.to_string());
        }
        ")" => {
            while let Some(top) = operators.top() {
                if *top == "(" {
                    operators.pop();
                    break;
                }

                output.push(top.to_string());
                operators.pop();
            }
        }
        "(" => operators.push(operator.to_string()),
        _ => output.push(operator.to_string()),
    }
}

pub fn postfix(expression: String) -> Stack<String> {
    let mut output: Stack<String> = Stack::new();
    let mut operators: Stack<String> = Stack::new();

    parse_expression(expression)
        .get()
        .iter()
        .for_each(|operator| handle_operator(operator.to_string(), &mut output, &mut operators));

    while let Some(operator) = operators.pop() {
        output.push(operator);
    }

    output
}
