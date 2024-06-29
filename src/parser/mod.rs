use crate::structs::Stack;

fn determine_character(operator: char, parsed: &mut Stack<String>, cache: &mut String) {
    if operator.is_whitespace() {
        return;
    }

    if operator.is_numeric() || operator == '.' {
        cache.push(operator);
        return;
    }

    if !cache.is_empty() {
        parsed.push(cache.clone());
        cache.clear();
    }

    parsed.push(operator.to_string());
}

pub fn parse_expression(expression: String) -> Stack<String> {
    let mut parsed: Stack<String> = Stack::new();
    let mut cache: String = String::new();

    expression
        .chars()
        .for_each(|operator| determine_character(operator, &mut parsed, &mut cache));

    if cache.is_empty() {
        return parsed;
    }

    parsed.push(cache.clone());

    parsed
}
