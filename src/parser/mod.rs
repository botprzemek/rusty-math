use crate::structs::Stack;

is_operator

pub fn parse_expression<'a>(expression: &str) -> Stack<&'a str> {
    let parsed: Stack<&str> = Stack::new();

    while let Some(operator) = expression.chars().next() {

    }

    parsed
}