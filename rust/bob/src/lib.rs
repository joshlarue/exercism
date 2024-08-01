/// Determines what response Bob should give
pub fn reply(message: &str) -> &str {
    let mut is_question = false;
    if message.trim().ends_with("?") {
        is_question = true;
    }

    let mut is_empty = false;
    if message.trim().is_empty() {
        is_empty = true;
    }

    let mut is_uppercase = false;
    let mut no_words = true;
    for ch in message.trim().chars() {
        if ch.is_ascii_punctuation() || ch.is_numeric() || ch.is_whitespace() || ch.is_uppercase() {
            is_uppercase = true;
        } else {
            is_uppercase = false;
            break;
        }

        // this loop makes sure a question with just numbers or non-alphabetical characters gets
        // processed as only a question instead of an uppercase question
        if no_words {
            if ch.is_ascii_punctuation() || ch.is_numeric() || ch.is_whitespace() || ch == '?' {
                no_words = true;
            } else {
                no_words = false;
            }
        }
    }

    if is_uppercase && is_question && !no_words {
        return "Calm down, I know what I'm doing!";
    }
    if is_empty {
        return "Fine. Be that way!";
    }
    if is_uppercase && !no_words {
        return "Whoa, chill out!";
    }
    if is_question {
        return "Sure.";
    }

    // if none of the above conditions match, "Whatever." is Bob's default response
    "Whatever."
}
