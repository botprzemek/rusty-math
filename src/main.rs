mod functions;
mod notations;
mod parser;
mod structures;

mod tests;

fn main() {
    notations::postfix("12+a*(b*c+d/e)".to_string());
    functions::quadratic(3.0, 5.0, -10.0);
    functions::horner(3, 5.0, &[1.0, 3.0, -6.0]);
}
