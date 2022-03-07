pub fn reply(message: &str) -> &str {
    let is_saying = message.trim().chars().count() > 0;
    let mut filtered_message = message.chars().filter(|c| c.is_alphabetic());
    let is_all_capitals = filtered_message.clone().count() > 0 && filtered_message.all(char::is_uppercase);
    let is_question = if let Some(char) = message.trim().chars().last() {
        char == '?'
    } else {
        false
    };

    if is_saying {
        if is_all_capitals {
            if is_question {
                "Calm down, I know what I'm doing!"
            } else {
                "Whoa, chill out!"
            }
        } else {
            if is_question {
                "Sure."
            } else {
                "Whatever."
            }
        }
    } else {
        "Fine. Be that way!"
    }
}
