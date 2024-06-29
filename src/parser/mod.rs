use crate::structs::Stack;

pub fn parse_expression(expression: String) -> Stack<String> {
    let mut list = expression.chars();

    let mut parsed: Stack<String> = Stack::new();
    let mut cache: Stack<String> = Stack::new();

    while let Some(operator) = list.next() {
        if operator.is_numeric() || operator == '.' {
            cache.push(operator.to_string());
        } else {
            if !operator.is_whitespace() {
                if !cache.is_empty() {
                    let number = cache.get().join("");

                    parsed.push(number);
                    cache.clear();
                }
                parsed.push(operator.to_string());
            }
        }
    }

    if cache.is_empty() {
        return parsed;
    }

    let number = cache.get().join("");
    parsed.push(number);

    parsed
}
