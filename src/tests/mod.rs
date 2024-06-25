use super::notations::postfix_expression;

pub fn postfix() {
    const EXPRESSION: &str = "3 + 4 * 2 / ( 1 - 5 ) ^ 2";
    const RESULT: &str = "3 4 2 * 1 5 - 2 ^ / +";

    assert_eq!(
        RESULT,
        postfix_expression(EXPRESSION).get().join(" "),
    );
}