struct Stack<T> {
    stack: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
      Stack {
          stack: Vec::new(),
      }
    }

    fn get(&self) -> &Vec<T> {
        &self.stack
    }

    fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }

    fn push(&mut self, item: T) {
        self.stack.push(item);
    }
}

fn transform_expression(expression: &str) -> Stack<String> {
    let input: Vec<&str> = expression.split(" ").collect();
    let mut output: Stack<String> = Stack::new();
    let mut operators: Stack<char> = Stack::new();

    for text in input {
        match text.parse::<f32>() {
            Ok(number) => {
                println!("Push \"{}\" to output", number);
                output.push(number.to_string());
            }
            Err(_) => {
                if let Some(operator) = text.chars().collect::<Vec<char>>().get(0) {
                    match operator {
                        '+' | '-' | '^' | '(' => {
                            println!("Push \"{}\" to stack", operator);
                            operators.push(*operator);
                        },
                        '/' | '*' | '%' => {
                            if let Some(operator) = operators.pop() {
                                if operator != '(' {
                                    println!("Push \"{}\" to output", operator);
                                    output.push(operator.to_string());
                                }
                            }

                            println!("Push \"{}\" to output", operator);
                            output.push(operator.to_string());
                        },
                        ')' => {
                            for _ in 0..operators.get().len() {
                                if let Some(operator) = operators.pop() {
                                    if operator == '(' {
                                        break;
                                    }

                                    println!("Push \"{}\" to output", operator);
                                    output.push(operator.to_string());
                                }
                            }
                        },
                        _ => {},
                    }
                }
            }
        }
    }

    for _ in 0..operators.get().len() {
        if let Some(operator) = operators.pop() {
            if operator != '(' {
                println!("Old. Push \"{}\" to output", operator);
                output.push(operator.to_string());
            }
        }
    }

    return output;
}

const TEST_EXPRESSION: &str = "3 + 4 * 2 / ( 1 − 5 ) ^ 2";
const TEST_RESULT: &str = "3 4 2 * 1 5 − 2 ^ / +";

fn test_postfix() {
    assert_eq!(
        TEST_RESULT,
        transform_expression(TEST_EXPRESSION).get().join(" "),
    );
}

fn main() {
    test_postfix();
}
