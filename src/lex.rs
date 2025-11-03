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
    Minus,
    Tilde,
    Plus,
    Asterisk,
    ForwardSlash,
    Percent,
    DoubleLeftAngleBracket,
    DoubleRightAngleBracket,
    Ampersand,
    Pipe,
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
    let empty_line_regex = Regex::new(r"^$").unwrap();
    let minus_regex = Regex::new(r"^-").unwrap();
    let decrement_operator_regex = Regex::new(r"^--").unwrap();
    let tilde_regex = Regex::new(r"^~").unwrap();
    let plus_regex = Regex::new(r"^\+").unwrap();
    let asterisk_regex = Regex::new(r"^\*").unwrap();
    let forward_slash_regex = Regex::new(r"^/").unwrap();
    let percent_regex = Regex::new(r"^%").unwrap();
    let double_left_angle_bracket_regex = Regex::new(r"^<<").unwrap();
    let double_right_angle_bracket_regex = Regex::new(r"^>>").unwrap();
    let ampersand_regex = Regex::new(r"^&").unwrap();
    let pipe_regex = Regex::new(r"^\|").unwrap();

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

            let res = empty_line_regex.find(&line[idx..]);
            if let Some(_) = res {
                // The removal of a newline character by the str.lines()` method means that a line
                // with only a newline character will have an empty string. In such a case, move to
                // the next line.
                traversed_entire_line = true;
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

            let res = decrement_operator_regex.find(&line[idx..]);
            if let Some(_) = res {
                panic!("Decrement operator is not supported yet");
            }

            let res = minus_regex.find(&line[idx..]);
            if let Some(_) = res {
                let token = Token::Minus;
                tokens.push(token);
                idx += 1;
                if idx == line.len() {
                    traversed_entire_line = true;
                }
                continue;
            }

            let res = tilde_regex.find(&line[idx..]);
            if let Some(_) = res {
                let token = Token::Tilde;
                tokens.push(token);
                idx += 1;
                if idx == line.len() {
                    traversed_entire_line = true;
                }
                continue;
            }

            let res = plus_regex.find(&line[idx..]);
            if let Some(_) = res {
                tokens.push(Token::Plus);
                idx += 1;
                if idx == line.len() {
                    traversed_entire_line = true;
                }
                continue;
            }

            if let Some(_) = asterisk_regex.find(&line[idx..]) {
                tokens.push(Token::Asterisk);
                idx += 1;
                if idx == line.len() {
                    traversed_entire_line = true;
                }
                continue;
            }

            if let Some(_) = forward_slash_regex.find(&line[idx..]) {
                tokens.push(Token::ForwardSlash);
                idx += 1;
                if idx == line.len() {
                    traversed_entire_line = true;
                }
                continue;
            }

            if let Some(_) = percent_regex.find(&line[idx..]) {
                tokens.push(Token::Percent);
                idx += 1;
                if idx == line.len() {
                    traversed_entire_line = true;
                }
                continue;
            }

            if double_left_angle_bracket_regex.find(&line[idx..]).is_some() {
                tokens.push(Token::DoubleLeftAngleBracket);
                idx += 2;
                if idx == line.len() {
                    traversed_entire_line = true;
                }
                continue;
            }

            if double_right_angle_bracket_regex
                .find(&line[idx..])
                .is_some()
            {
                tokens.push(Token::DoubleRightAngleBracket);
                idx += 2;
                if idx == line.len() {
                    traversed_entire_line = true;
                    continue;
                }
            }

            if ampersand_regex.find(&line[idx..]).is_some() {
                tokens.push(Token::Ampersand);
                idx += 1;
                if idx == line.len() {
                    traversed_entire_line = true;
                    continue;
                }
            }

            if pipe_regex.find(&line[idx..]).is_some() {
                tokens.push(Token::Pipe);
                idx += 1;
                if idx == line.len() {
                    traversed_entire_line = true;
                    continue;
                }
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

    #[test]
    fn get_correct_tokens_despite_newline_characters() {
        let source_code_string = "
int main() {
    return 2;

}
";
        let expected_tokens = vec![
            Token::IntKeyword,
            Token::Identifier("main".to_string()),
            Token::OpenParenthesis,
            Token::CloseParenthesis,
            Token::OpenBrace,
            Token::ReturnKeyword,
            Token::NumericConstant(2),
            Token::Semicolon,
            Token::CloseBrace,
        ];
        let tokens = lex(source_code_string);
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn minus_character_token_is_created() {
        let source_code_string = "int main() {return -2;}";
        let expected_tokens = vec![
            Token::IntKeyword,
            Token::Identifier("main".to_string()),
            Token::OpenParenthesis,
            Token::CloseParenthesis,
            Token::OpenBrace,
            Token::ReturnKeyword,
            Token::Minus,
            Token::NumericConstant(2),
            Token::Semicolon,
            Token::CloseBrace,
        ];
        let tokens = lex(source_code_string);
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    #[should_panic(expected = "Decrement operator is not supported yet")]
    fn panic_if_decrement_operator_detected() {
        let source_code_string = "int main() {return --2;}";
        _ = lex(source_code_string);
    }

    #[test]
    fn tilde_token_is_created() {
        let source_code_string = "int main() {return ~(-2);}";
        let expected_tokens = vec![
            Token::IntKeyword,
            Token::Identifier("main".to_string()),
            Token::OpenParenthesis,
            Token::CloseParenthesis,
            Token::OpenBrace,
            Token::ReturnKeyword,
            Token::Tilde,
            Token::OpenParenthesis,
            Token::Minus,
            Token::NumericConstant(2),
            Token::CloseParenthesis,
            Token::Semicolon,
            Token::CloseBrace,
        ];
        let tokens = lex(source_code_string);
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn plus_character_token_is_created() {
        let source_code_string = "int main() {return 2+";
        let expected_tokens = vec![
            Token::IntKeyword,
            Token::Identifier("main".to_string()),
            Token::OpenParenthesis,
            Token::CloseParenthesis,
            Token::OpenBrace,
            Token::ReturnKeyword,
            Token::NumericConstant(2),
            Token::Plus,
        ];
        let tokens = lex(source_code_string);
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn asterisk_character_token_is_created() {
        let source_code_string = "int main() {return 2*";
        let expected_tokens = vec![
            Token::IntKeyword,
            Token::Identifier("main".to_string()),
            Token::OpenParenthesis,
            Token::CloseParenthesis,
            Token::OpenBrace,
            Token::ReturnKeyword,
            Token::NumericConstant(2),
            Token::Asterisk,
        ];
        let tokens = lex(source_code_string);
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn forward_slash_character_token_is_created() {
        let source_code_string = "int main() {return 2/";
        let expected_tokens = vec![
            Token::IntKeyword,
            Token::Identifier("main".to_string()),
            Token::OpenParenthesis,
            Token::CloseParenthesis,
            Token::OpenBrace,
            Token::ReturnKeyword,
            Token::NumericConstant(2),
            Token::ForwardSlash,
        ];
        let tokens = lex(source_code_string);
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn percent_character_token_is_created() {
        let source_code_string = "int main() {return 2%";
        let expected_tokens = vec![
            Token::IntKeyword,
            Token::Identifier("main".to_string()),
            Token::OpenParenthesis,
            Token::CloseParenthesis,
            Token::OpenBrace,
            Token::ReturnKeyword,
            Token::NumericConstant(2),
            Token::Percent,
        ];
        let tokens = lex(source_code_string);
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn double_left_angle_bracket_token_is_created() {
        let source_code_string = "<<";
        let expected_tokens = vec![Token::DoubleLeftAngleBracket];
        let tokens = lex(source_code_string);
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn double_right_angle_bracket_token_is_created() {
        let source_code_string = ">>";
        let expected_tokens = vec![Token::DoubleRightAngleBracket];
        let tokens = lex(source_code_string);
        assert_eq!(tokens, expected_tokens);
    }

    #[test]
    fn ampersand_token_is_created() {
        assert_eq!(lex("&"), vec![Token::Ampersand]);
    }

    #[test]
    fn pipe_token_is_created() {
        assert_eq!(lex("|"), vec![Token::Pipe]);
    }
}
