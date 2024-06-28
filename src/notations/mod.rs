use crate::structs::Stack;

fn handle_operator(operation: &str, output: &mut Stack<String>, operators: &mut Stack<char>) {
    if let Some(operator) = operation.chars().next() {
        match operator {
            '(' => {
                operators.push(operator);
            }
            ')' => {
                while let Some(top) = operators.top() {
                    if *top == '(' {
                        operators.pop();
                        break;
                    }

                    output.push((*top).to_string());
                    operators.pop();
                }
            }
            '+' | '-' => {
                while let Some(top) = operators.top() {
                    match *top {
                        '+' | '-' | '*' | '/' | '%' | '^' => {
                            output.push((*top).to_string());
                            operators.pop();
                        }
                        _ => {
                            break;
                        }
                    }
                }

                operators.push(operator);
            }
            '*' | '/' | '%' => {
                while let Some(top) = operators.top() {
                    match *top {
                        '*' | '/' | '%' | '^' => {
                            output.push((*top).to_string());
                            operators.pop();
                        }
                        _ => {
                            break;
                        }
                    }
                }

                operators.push(operator);
            }
            '^' => {
                while let Some(top) = operators.top() {
                    if *top != '^' {
                        break;
                    }

                    output.push((*top).to_string());
                    operators.pop();
                }

                operators.push(operator);
            }
            _ => output.push(operator.to_string()),
        }
    }
}

pub fn postfix(expression: &str) -> Stack<String> {
    let mut output: Stack<String> = Stack::new();
    let mut operators: Stack<char> = Stack::new();

    let input: Vec<&str> = expression.split_whitespace().collect();

    for operation in input {
        match operation.parse::<f32>() {
            Ok(number) => output.push(number.to_string()),
            Err(_) => handle_operator(operation, &mut output, &mut operators),
        }
    }

    while let Some(operator) = operators.pop() {
        output.push(operator.to_string());
    }

    output
}
