use regex::Regex;

static INT_KEYWORD_LEN: usize = 3;
static RETURN_KEYWORD_LEN: usize = 6;

#[derive(Debug, PartialEq)]
pub enum Token {
    IntKeyword,
    Identifier(String),
    OpenParenthesis,
    CloseParenthesis,
    OpenBrace,
    CloseBrace,
    ReturnKeyword,
    NumericConstant(u8),
    Semicolon,
}

pub fn lex(text: &str) -> Vec<Token> {
    let int_keyword_regex = Regex::new(r"^int\b").unwrap();
    let identifier_regex = Regex::new(r"^[a-zA-Z]\w*\b").unwrap();
    let whitespace_regex = Regex::new(r"^\s+").unwrap();
    let open_parenthesis_regex = Regex::new(r"^\(").unwrap();
    let close_parenthesis_regex = Regex::new(r"^\)").unwrap();
    let open_brace_regex = Regex::new(r"^\{").unwrap();
    let close_brace_regex = Regex::new(r"^\}").unwrap();
    let return_keyword_regex = Regex::new(r"^return\b").unwrap();
    let numeric_constant_regex = Regex::new(r"^[0-9]+\b").unwrap();
    let semicolon_regex = Regex::new(r"^;").unwrap();

    let mut tokens: Vec<Token> = vec![];

    for line in text.lines() {
        let mut traversed_entire_line = false;
        let mut idx = 0;

        while !traversed_entire_line {
            let res = whitespace_regex.find(&line[idx..]);
            if let Some(mat) = res {
                // Advance past the whitespace
                idx += mat.end();
                continue;
            }

            let res = int_keyword_regex.find(&line[idx..]);
            if let Some(_) = res {
                let token = Token::IntKeyword;
                tokens.push(token);

                // Advance past the substring that a match was found for the `int` keyword
                idx += INT_KEYWORD_LEN;
                if idx == line.len() {
                    traversed_entire_line = true;
                }
                continue;
            }

            let res = return_keyword_regex.find(&line[idx..]);
            if let Some(_) = res {
                let token = Token::ReturnKeyword;
                tokens.push(token);
                idx += RETURN_KEYWORD_LEN;
                if idx == line.len() {
                    traversed_entire_line = true;
                }
                continue;
            }

            let res = identifier_regex.find(&line[idx..]);
            if let Some(mat) = res {
                let token = Token::Identifier(mat.as_str().to_string());
                tokens.push(token);
                idx += mat.end();
                if idx == line.len() {
                    traversed_entire_line = true;
                }
                continue;
            }

            let res = open_parenthesis_regex.find(&line[idx..]);
            if let Some(_) = res {
                let token = Token::OpenParenthesis;
                tokens.push(token);
                idx += 1;
                if idx == line.len() {
                    traversed_entire_line = true;
                }
                continue;
            }

            let res = close_parenthesis_regex.find(&line[idx..]);
            if let Some(_) = res {
                let token = Token::CloseParenthesis;
                tokens.push(token);
                idx += 1;
                if idx == line.len() {
                    traversed_entire_line = true;
                }
                continue;
            }

            let res = open_brace_regex.find(&line[idx..]);
            if let Some(_) = res {
                let token = Token::OpenBrace;
                tokens.push(token);
                idx += 1;
                if idx == line.len() {
                    traversed_entire_line = true;
                }
                continue;
            }

            let res = close_brace_regex.find(&line[idx..]);
            if let Some(_) = res {
                let token = Token::CloseBrace;
                tokens.push(token);
                idx += 1;
                if idx == line.len() {
                    traversed_entire_line = true;
                }
                continue;
            }

            let res = numeric_constant_regex.find(&line[idx..]);
            if let Some(mat) = res {
                let value = mat
                    .as_str()
                    .parse::<u8>()
                    .expect("Match from regex should remove all whitespace");
                let token = Token::NumericConstant(value);
                tokens.push(token);
                idx += mat.end();
                if idx == line.len() {
                    traversed_entire_line = true;
                }
                continue;
            }

            let res = semicolon_regex.find(&line[idx..]);
            if let Some(_) = res {
                let token = Token::Semicolon;
                tokens.push(token);
                idx += 1;
                if idx == line.len() {
                    traversed_entire_line = true;
                }
                continue;
            }

            // No match was found, so the string contains either:
            // - valid C code, but not yet supported
            // - invalid C code
            //
            // These cases should be handled differently, but for now, panic for both
            panic!(
                "No match found for the following substring: {}",
                &line[idx..]
            )
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

    #[test]
    fn create_int_keyword_and_main_identifier_tokens() {
        let source_code_string = "int main";
        let expected_tokens = vec![Token::IntKeyword, Token::Identifier("main".to_string())];
        let tokens = lex(source_code_string);
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    #[should_panic(expected = "No match found for the following substring: ?")]
    fn panic_if_no_match_found_for_substring() {
        let source_code_string = "?";
        lex(source_code_string);
    }

    #[test]
    fn open_parenthesis_token_is_created() {
        let source_code_string = "int main(";
        let expected_last_token = Token::OpenParenthesis;
        let tokens = lex(source_code_string);
        assert_eq!(tokens[tokens.len() - 1], expected_last_token);
    }

    #[test]
    fn close_parenthesis_token_is_created() {
        let source_code_string = "int main()";
        let expected_last_token = Token::CloseParenthesis;
        let tokens = lex(source_code_string);
        assert_eq!(tokens[tokens.len() - 1], expected_last_token);
    }

    #[test]
    fn open_brace_token_is_created() {
        let source_code_string = "int main() {";
        let expected_last_token = Token::OpenBrace;
        let tokens = lex(source_code_string);
        assert_eq!(tokens[tokens.len() - 1], expected_last_token);
    }

    #[test]
    fn close_brace_token_is_created() {
        let source_code_string = "int main() {}";
        let expected_last_token = Token::CloseBrace;
        let tokens = lex(source_code_string);
        assert_eq!(tokens[tokens.len() - 1], expected_last_token);
    }

    #[test]
    fn return_keyword_token_is_created() {
        let source_code_string = "int main() {return";
        let expected_last_token = Token::ReturnKeyword;
        let tokens = lex(source_code_string);
        assert_eq!(tokens[tokens.len() - 1], expected_last_token);
    }

    #[test]
    fn numeric_constant_token_is_created_with_correct_value() {
        let source_code_string = "int main() {return 2";
        let expected_last_token = Token::NumericConstant(2);
        let tokens = lex(source_code_string);
        assert_eq!(tokens[tokens.len() - 1], expected_last_token);
    }

    #[test]
    fn semicolon_token_is_created() {
        let source_code_string = "int main() {return 2;";
        let expected_last_token = Token::Semicolon;
        let tokens = lex(source_code_string);
        assert_eq!(tokens[tokens.len() - 1], expected_last_token);
    }
}
