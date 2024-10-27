use regex::Regex;

static INT_KEYWORD_LEN: usize = 3;

#[derive(Debug, PartialEq)]
pub enum Token {
    IntKeyword,
}

pub fn lex(text: &str) -> Vec<Token> {
    let int_keyword_regex = Regex::new(r"^int\b").unwrap();

    let mut tokens: Vec<Token> = vec![];

    for line in text.lines() {
        let mut traversed_entire_line = false;
        let mut idx = 0;

        while !traversed_entire_line {
            let Some(_) = int_keyword_regex.find(&line[idx..]) else {
                // Handle if no match is found
                todo!();
            };
            let token = Token::IntKeyword;
            tokens.push(token);

            // Advance past the substring that a match was found for the `int` keyword
            idx += INT_KEYWORD_LEN;
            if idx == line.len() {
                traversed_entire_line = true;
            }
        }
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_int_keyword_token_when_found_at_start_of_string() {
        let source_code_string = "int";
        let expected_tokens = vec![Token::IntKeyword];
        let tokens = lex(source_code_string);
        assert_eq!(tokens, expected_tokens);
    }
}
