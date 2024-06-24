fn derivative(n: i32) -> i32 {
    // f(x) = 3 * x ^ 2
    // f'(x) = 3 * 2 * x ^ 1

    let mut x = 2;

    for number in 1..n - 1 {
        if number == n {
            break;
        }

        x *= x;

        println!("X: {}", x);
    }

    return x * n;
}

// struct Stack<T> {
//     stack: Vec<T>,
// }

// impl<T> Stack<T> {
//     fn new() -> Self {
//       Stack { stack: Vec::new() }
//     }

//     fn empty(&mut self) {
//         self.stack.clear();
//     }

//     fn pop(&mut self) -> Option<T> {
//         self.stack.pop()
//     }

//     fn push(&mut self, item: T) {
//         self.stack.push(item);
//     }

//     fn top(&mut self) -> Option<&T> {
//         self.stack.first()
//     }
// }

// fn transform_expression(expression: &str) -> Stack<String> {
//     // (2+3)*5  =>  ["2", "3", "+", "5", "*"]

//     let input: Vec<&str> = expression.split(' ').collect();
//     let mut output: Stack<String> = Stack::new();
//     let mut operators: Stack<char> = Stack::new();

//     for value in input {
//         let test: Vec<char> = value.chars().collect();

//         if test.len() == 1 {
//             match test.get(0) {
//                 None => (),
//                 Some(operator) => {
//                     match operator {
//                         '+' | '-' | '*' | '/' | '^' | '%' | '(' | ')' => {
//                             operators.push(*operator);
//                         },
//                         _ => (),
//                     }
//                 },
//             }
//         }

//         else {
//             let number = value.trim().parse::<i64>();

//             match number {
//                 Ok(ok) => {
//                     output.push(ok);
//                     print!("{:?}", output)
//                 },
//                 Err(error) => print!("{}", error),
//             }
//         }
//     }

//     return output;
// }

// fn calculate_postfix(expression: &str) -> i32 {
//     let mut result = 0;

//     for value in transform_expression(expression).filter(|value| *value != ' ') {
//         match value {
//             '0'..='9' => operation.push(value),
//             '+' | '-' | '*' | '/' | '^' | '%' => print!("operator, "),
//             _ => (),
//         }

//         result += 1;
//     }

//     return result;
// }

// const TEST_EXPRESSION: &str = "(2+3)*5";

// fn test_postfix() {
//     let result = calculate_postfix(TEST_EXPRESSION);
//     print!("Test Result: {}", result);
//     assert!(result == 25);
// }

fn main() {
    println!("Pochodna z x^3: {}", pochodna(3));
}
