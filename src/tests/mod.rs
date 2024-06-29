#[test]
fn parser() {
    const EXPRESSION: &str = "a + 12 .34~ * (5 + 6_) / 7.8 9@@@";
    const RESULT: &str = "a+12.34*(5+6)/7.89";
    assert_eq!(
        RESULT.to_string(),
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
