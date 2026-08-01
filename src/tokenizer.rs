use std::collections::HashMap;

pub fn tokenize(file_contents: String) -> (Vec<String>, Vec<String>) {
    let res_words = HashMap::from([
        ("and", "AND"),
        ("class", "CLASS"),
        ("else", "ELSE"),
        ("false", "FALSE"),
        ("for", "FOR"),
        ("fun", "FUN"),
        ("if", "IF"),
        ("nil", "NIL"),
        ("or", "OR"),
        ("print", "PRINT"),
        ("return", "RETURN"),
        ("super", "SUPER"),
        ("this", "THIS"),
        ("true", "TRUE"),
        ("var", "VAR"),
        ("while", "WHILE"),
        ("print", "PRINT"),
        ("var", "VAR"),
    ]);
    let mut str_iter = file_contents.chars().peekable();
    let mut new_line: i32 = 1; //  have to do something with this that allows the next thing to see it
    let mut result: Vec<String> = Vec::new();
    let mut eresult: Vec<String> = Vec::new();

    if !file_contents.is_empty() {
        while let Some(ch) = str_iter.next() {
            // Option<char>
            match ch {
                '(' => result.push("LEFT_PAREN ( null".to_string()),
                ')' => result.push("RIGHT_PAREN ) null".to_string()),
                '{' => result.push("LEFT_BRACE { null".to_string()),
                '}' => result.push("RIGHT_BRACE } null".to_string()),
                '.' => result.push("DOT . null".to_string()),
                ',' => result.push("COMMA , null".to_string()),
                '+' => result.push("PLUS + null".to_string()),
                '*' => result.push("STAR * null".to_string()),
                '-' => result.push("MINUS - null".to_string()),
                '[' => result.push("LEFT_SQUARE [ null".to_string()),
                ']' => result.push("RIGHT_SQUARE ] null".to_string()),
                '/' => {
                    if str_iter.peek() == Some(&'/') {
                        while let Some(new_ch) = str_iter.next() {
                            if new_ch == '\n' {
                                new_line += 1;
                                break;
                            }
                        }
                    } else {
                        result.push("SLASH / null".to_string());
                    }
                }
                '"' => {
                    let mut lexeme = '"'.to_string(); // takes &temp and creates new mem add with modifiable string
                    let mut literal = String::new();

                    while let Some(new_ch) = str_iter.next() {
                        if new_ch == '"' {
                            lexeme.push(new_ch);
                            break;
                        };
                        lexeme.push(new_ch); // "abcd...
                        literal.push(new_ch); //abcd... 
                    }
                    if !lexeme.ends_with('"') {
                        eresult.push(format!("[line {}] Error: Unterminated string.", new_line));
                    } else {
                        result.push(format!("STRING {} {}", lexeme, literal));
                    }
                }
                ';' => result.push("SEMICOLON ; null".to_string()),
                '=' => {
                    if str_iter.peek() == Some(&'=') {
                        // does NOT consume the next value. & finds address of equal, * refrences the address created by &
                        let _: Option<char> = str_iter.next();
                        result.push("EQUAL_EQUAL == null".to_string());
                    } else {
                        result.push("EQUAL = null".to_string());
                    }
                }
                '!' => {
                    if str_iter.peek() == Some(&'=') {
                        let _: Option<char> = str_iter.next();
                        result.push("BANG_EQUAL != null".to_string());
                    } else {
                        result.push("BANG ! null".to_string());
                    }
                }
                '>' => {
                    if str_iter.peek() == Some(&'=') {
                        let _: Option<char> = str_iter.next();
                        result.push("GREATER_EQUAL >= null".to_string());
                    } else {
                        result.push("GREATER > null".to_string());
                    }
                }
                '<' => {
                    if str_iter.peek() == Some(&'=') {
                        // automatically derefences equal, so chars are compared.
                        let _: Option<char> = str_iter.next();
                        result.push("LESS_EQUAL <= null".to_string());
                    } else {
                        result.push("LESS < null".to_string());
                    }
                }
                ' ' | '\t' => {}
                '\n' => {
                    new_line += 1;
                }
                _ => {
                    if ch.is_digit(10) {
                        // finding numbers
                        let mut literal = ch.to_string();
                        while let Some(new_ch) = str_iter.peek() {
                            //Option<&char>
                            if new_ch.is_digit(10) || *new_ch == '.' {
                                literal.push(*new_ch);
                                let _: Option<char> = str_iter.next();
                            } else {
                                break;
                            }
                        }
                        if let Ok(value) = literal.parse::<f64>() {
                            result.push(format!("NUMBER {} {:?}", literal, value));
                        };
                    } else if ch == '_' || ch.is_ascii_alphabetic() {
                        //strings
                        let mut identifier: String = ch.to_string();
                        while let Some(new_ch) = str_iter.peek() {
                            if !new_ch.is_digit(10)
                                && !(*new_ch == '_')
                                && !new_ch.is_ascii_alphabetic()
                            {
                                break;
                            }
                            identifier.push(*new_ch);
                            let _: Option<char> = str_iter.next();
                        } // typically pushes identifer in, but if we want print then want to get our string afger 

                        if res_words.contains_key(&*identifier) {
                            let reference = res_words[&*identifier];
                            result.push(format!("{} {} null", reference, identifier));
                        } else {
                            result.push(format!("IDENTIFIER {} null", identifier));
                        }
                    } else {
                        eresult.push(format!(
                            "[line {}] Error: Unexpected character: {}",
                            new_line, ch
                        ));
                    }
                }
            } // match ends. 
        } // while ends
        result.push("EOF  null".to_string());
        return (result, eresult);
    } // if ends
    result.push("EOF  null".to_string());
    return (result, eresult);
}
