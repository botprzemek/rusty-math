use crate::structs::Stack;

fn get_precedence(operator: &str) -> u8 {
    match operator {
        "+" | "-" => 1,
        "*" | "/" | "%" => 2,
        "^" => 3,
        _ => 0,
    }
}

fn handle_operator<'a>(operator: &'a str, output: &mut Stack<&'a str>, operators: &mut Stack<&'a str>) {
    match operator {
        "+" | "-" | "*" | "/" | "%" | "^" => {
            while let Some(top) = operators.top() {
                if get_precedence(*top) < get_precedence(operator) {
                    break;
                }

                output.push(*top);
                operators.pop();
            }

            operators.push(operator);
        }
        ")" => {
            while let Some(top) = operators.top() {
                if *top == "(" {
                    operators.pop();
                    break;
                }

                output.push(*top);
                operators.pop();
            }
        }
        "(" => operators.push(operator),
        _ => output.push(operator),
    }
}

pub fn postfix(expression: &str) -> Stack<&str> {
    let mut output: Stack<&str> = Stack::new();
    let mut operators: Stack<&str> = Stack::new();

    expression
        .split_whitespace()
        .for_each(| operator| handle_operator(operator, &mut output, &mut operators));

    while let Some(operator) = operators.pop() {
        output.push(operator);
    }

    output
}
