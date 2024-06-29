#[test]
fn parser() {
    const EXPRESSION: &str = "((2+7)/3+(14-3)*4)/2";
    assert_eq!(
        EXPRESSION.to_string(),
        crate::parser::parse_expression(EXPRESSION.to_string())
            .get()
            .join(""),
    );
}

#[test]
fn postfix() {
    const EXPRESSION: &str = "((2+7)/3+(14-3)*4)/2";
    const RESULT: &str = "2 7 + 3 / 14 3 - 4 * + 2 /";

    assert_eq!(
        RESULT.to_string(),
        crate::notations::postfix(EXPRESSION.to_string())
            .get()
            .join(" "),
    );
}
