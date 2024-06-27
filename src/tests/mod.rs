#[test]
fn postfix() {
    const EXPRESSION: &str = "( ( 2 + 7 ) / 3 + ( 14 - 3 ) * 4 ) / 2";
    const RESULT: &str = "2 7 + 3 / 14 3 - 4 * + 2 /";

    assert_eq!(
        RESULT,
        crate::notations::postfix(EXPRESSION).get().join(" "),
    );
}
