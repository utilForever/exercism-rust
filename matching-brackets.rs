pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::new();

    for ch in string.chars() {
        match ch {
            '(' | '{' | '[' => stack.push(ch),
            ')' | '}' | ']' => {
                if stack.is_empty() {
                    return false;
                }

                let bracket = if ch == ')' {
                    '('
                } else if ch == '}' {
                    '{'
                } else {
                    '['
                };

                if *stack.last().unwrap() != bracket {
                    return false;
                }

                stack.pop();
            }
            _ => (),
        }
    }

    stack.is_empty()
}
