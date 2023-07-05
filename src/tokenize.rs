

#[derive(Debug,PartialEq,Clone)]
pub enum Token {
    Number(i32),
    Plus,
    Minus,
    Asterisk,
    Slash,
    Power,
    LParen,
    RParen,
}


pub fn tokenize(s:String) -> Vec<Token> {
    
    let mut pos = 0;

    let mut tokens = Vec::new();

    let mut buffer = None;

    while pos < s.len() {

        let c = s.chars().nth(pos).unwrap();

        match c {
            '0'..='9' => {
                buffer = Some(buffer.unwrap_or(0) * 10 + c.to_digit(10).unwrap() as i32);
            }

            '+' => {
                if let Some(n) = buffer {
                    tokens.push(Token::Number(n));
                }
                tokens.push(Token::Plus);
                buffer = None;
            }

            '-' => {
                if let Some(n) = buffer {
                    tokens.push(Token::Number(n));
                }
                tokens.push(Token::Minus);
                buffer = None;
            }

            '*' => {
                if let Some(n) = buffer {
                    tokens.push(Token::Number(n));
                }

                // 次の文字を先読みする

                if let Some('*') = s.chars().nth(pos + 1) {
                    tokens.push(Token::Power);
                    pos += 1;
                } else {
                    tokens.push(Token::Asterisk);
                }
                buffer = None;
            }

            '/' => {
                if let Some(n) = buffer {
                    tokens.push(Token::Number(n));
                }
                tokens.push(Token::Slash);
                buffer = None;
            }

            '(' => {
                
                tokens.push(Token::LParen);
                buffer = None;
            }

            ')' => {
                if let Some(n) = buffer {
                    tokens.push(Token::Number(n));
                }
                tokens.push(Token::RParen);
                buffer = None;
            }

            // space
            ' ' => {
                
            }

            _ => {
                panic!("トークナイズできません: {}", c);
            }

        }

        pos += 1;

    }

    if let Some(n) = buffer {
        tokens.push(Token::Number(n));
    }

    tokens
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize1() {
        let expression = "3 * 2 + 1 + 5 ";

        let tokens = tokenize(expression.to_string());

        assert_eq!(tokens, vec![
            Token::Number(3),
            Token::Asterisk,
            Token::Number(2),
            Token::Plus,
            Token::Number(1),
            Token::Plus,
            Token::Number(5),
        ]);
    }

    #[test]
    fn test_tokenize2() {
        let expression = "3 * (2 ** 3) + 1 + 5 ";

        let tokens = tokenize(expression.to_string());

        assert_eq!(tokens, vec![
            Token::Number(3),
            Token::Asterisk,
            Token::LParen,
            Token::Number(2),
            Token::Power,
            Token::Number(3),
            Token::RParen,
            Token::Plus,
            Token::Number(1),
            Token::Plus,
            Token::Number(5),
        ]);
    }
}