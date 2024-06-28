mod functions;
mod notations;
mod structs;

mod tests;

fn main() {
    let expression = "3 * x ^ 2 + 5 * x + 10";
    let result = notations::postfix(expression);

    println!("Result: {}", result.get().join(" "));

    functions::quadratic(3.0, 5.0, 10.0);
}
