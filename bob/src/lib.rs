/*
- **"Sure."**
  This is his response if you ask him a question, such as "How are you?"
  The convention used for questions is that it ends with a question mark.
- **"Whoa, chill out!"**
  This is his answer if you YELL AT HIM.
  The convention used for yelling is ALL CAPITAL LETTERS.
- **"Calm down, I know what I'm doing!"**
  This is what he says if you yell a question at him.
- **"Fine. Be that way!"**
  This is how he responds to silence.
  The convention used for silence is nothing, or various combinations of whitespace characters.
- **"Whatever."**
  This is what he answers to anything else.
*/

pub fn reply(message: &str) -> &str {
    let trimmed = message.trim();
    if trimmed.is_empty() {
        "Fine. Be that way!"
    } else if trimmed.ends_with('?') && trimmed.to_uppercase() == trimmed {
        if trimmed.bytes().any(|b| b.is_ascii_alphabetic()) {
            "Calm down, I know what I'm doing!"
        } else {
            "Sure."
        }
    } else if trimmed.ends_with('?') {
        "Sure."
    } else if trimmed.to_uppercase() == trimmed {
        if trimmed.bytes().any(|b| b.is_ascii_alphabetic()) {
            "Whoa, chill out!"
        } else {
            "Whatever."
        }
    } else {
        "Whatever."
    }
}
