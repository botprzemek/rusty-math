use crate::structs::Stack;

pub fn postfix(expression: &str) -> String {
    let input: Vec<&str> = expression.split(' ').collect();

    let mut output: Stack<String> = Stack::new();
    let mut operators: Stack<char> = Stack::new();

    for text in input {
        match text.parse::<f32>() {
            Ok(number) => {
                output.push(number.to_string());
            }
            Err(_) => {
                if let Some(operator) = text.chars().collect::<Vec<char>>().first() {
                    output.push(operator.to_string());
                }
            }
        }
    }

    for _ in 0..operators.get().len() {
        if let Some(operator) = operators.pop() {
            output.push(operator.to_string());
        }
    }

    output.get().join(" ")
}

// mod tests {
//     #[test]
//     fn postfix() {
//         const EXPRESSION: &str = "3 + 4 * 2 / ( 1 - 5 ) ^ 2";
//         const RESULT: &str = "3 4 2 * 1 5 - 2 ^ / +";
    
//         assert_eq!(
//             RESULT,
//             crate::notations::postfix(EXPRESSION),
//         );
//     }
// }