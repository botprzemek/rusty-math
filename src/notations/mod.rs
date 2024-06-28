use crate::structs::Stack;

fn get_precedence(operator: char) -> u8 {
    match operator {
        '+' | '-' => 1,
        '*' | '/' | '%' => 2,
        '^' => 3,
        _ => 0,
    }
}

fn handle_operator(operator: char, output: &mut Stack<String>, operators: &mut Stack<char>) {
    match operator {
        '+' | '-' | '*' | '/' | '%' | '^' => {
            while let Some(top) = operators.top() {
                if get_precedence(*top) < get_precedence(operator) {
                    break;
                }

                output.push((*top).to_string());
                operators.pop();
            }

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
        '(' => operators.push(operator),
        _ => output.push(operator.to_string()),
    }
}

pub fn postfix(expression: &str) -> Stack<String> {
    let mut output: Stack<String> = Stack::new();
    let mut operators: Stack<char> = Stack::new();

    for token in expression.split_whitespace() {
        match token.parse::<f32>() {
            Ok(number) => output.push(number.to_string()),
            Err(_) => {
                if let Some(operator) = token.chars().next() {
                    handle_operator(operator, &mut output, &mut operators)
                }
            },
        }
    }

    while let Some(operator) = operators.pop() {
        output.push(operator.to_string());
    }

    output
}
