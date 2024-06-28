mod notations;
mod structs;

mod tests;

fn main() {
    let expression = "12 + a * ( b * c + d / e )";
    let result = notations::postfix(expression);

    println!("Result: {}", result.get().join(" "));
}
