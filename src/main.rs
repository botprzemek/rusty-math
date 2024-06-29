mod functions;
mod notations;
mod parser;
mod structs;

mod tests;

fn main() {
    notations::postfix("12+a*(b*c+d/e)".to_string());
    functions::quadratic(3.0, 5.0, -10.0);
}
