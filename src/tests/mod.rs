#[test]
fn postfix() {
    const EXPRESSION: &str = "3 + 4 * 2 / ( 1 - 5 ) ^ 2";
    const RESULT: &str = "3 4 2 * 1 5 - 2 ^ / +";

    assert_eq!(
        RESULT,
        crate::notations::postfix(EXPRESSION),
    );
}