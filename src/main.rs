mod notations;
mod structs;

mod tests;

fn main() {
    let expression = "d r 3 + 4 * 2 / ( 1 - 5 ) ^ 2";
    let result = notations::postfix(expression);

    println!("Result: {}", result.get().join(" "));
}
