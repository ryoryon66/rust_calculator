
// 四則演算のパーサーをかく
use std::cell::Cell;
use std::io::{self, Write};

#[derive(Debug,PartialEq,Clone)]
enum Token {
    Number(i32),
    Plus,
    Minus,
    Asterisk,
    Slash,
    Power,
    LParen,
    RParen,
}

#[derive(Debug,PartialEq)]
enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Power,
}

#[derive(Debug,PartialEq)]
enum Ast {
    Number(i32),
    BinOp(Box<Ast>, BinOp, Box<Ast>),
}

fn tokenize(s:String) -> Vec<Token> {
    
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

struct Parser {
    tokens: Vec<Token>,
    pos : Cell<usize>,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            pos: Cell::new(0),
        }
    }

    fn consume(&self,token:Token) -> bool {
        if self.pos.get() >= self.tokens.len() {
            return false;
        }
        if self.tokens[self.pos.get()] == token {
            self.pos.set(self.pos.get() + 1);
            true
        } else {
            false
        }
    }

    fn peek(&self) -> Token {
        self.tokens[self.pos.get()].clone()
    }

    fn parse(&self) -> Ast {
        self.expr()
    }

    fn expr(&self) -> Ast {
        let mut node = self.term();

        loop {
            if self.consume(Token::Plus) {
                let lhs = node;
                let rhs = self.term();
                node = Ast::BinOp(Box::new(lhs), BinOp::Add, Box::new(rhs));
            } else if self.consume(Token::Minus) {
                let lhs = node;
                let rhs = self.term();
                node = Ast::BinOp(Box::new(lhs), BinOp::Sub, Box::new(rhs));
            } else {
                break;
            }

        }
        node
    }
    
    fn term(&self) -> Ast {
        let mut node = self.power();

        loop {
            if self.consume(Token::Asterisk) {
                let lhs = node;
                let rhs = self.power();
                node = Ast::BinOp(Box::new(lhs), BinOp::Mul, Box::new(rhs));
            } else if self.consume(Token::Slash) {
                let lhs = node;
                let rhs = self.power();
                node = Ast::BinOp(Box::new(lhs), BinOp::Div, Box::new(rhs));
            } else {
                break;
            }
        }
        node
    }

    fn power(&self) -> Ast {
        let mut node = self.factor();

        // べき乗は右結合であることに注意する
        if self.consume(Token::Power) {
            let lhs = node;
            let rhs = self.power();
            node = Ast::BinOp(Box::new(lhs), BinOp::Power, Box::new(rhs));
        }

        node
    }

    fn factor(&self) -> Ast {
        if self.consume(Token::LParen) {
            let node = self.expr();
            assert!(self.consume(Token::RParen));
            node
        } else {
            match self.peek() {
                Token::Number(n) => {
                    self.pos.set(self.pos.get() + 1);
                    Ast::Number(n)
                }
                _ => {
                    panic!("factorではないトークンです: {:?}", self.peek());
                }
            }
        }
    }


}


fn eval (ast : Ast) -> i32 {
    match ast {
        Ast::Number(n) => n,
        Ast::BinOp(lhs, op, rhs) => {
            let lhs = eval(*lhs);
            let rhs = eval(*rhs);
            match op {
                BinOp::Add => lhs + rhs,
                BinOp::Sub => lhs - rhs,
                BinOp::Mul => lhs * rhs,
                BinOp::Div => lhs / rhs,
                BinOp::Power => lhs.pow(rhs as u32),
            }
        }
    }
}

fn calculate(s:String) -> i32 {
    let tokens = tokenize(s);
    let parser = Parser::new(tokens);
    let ast = parser.parse();
    eval(ast)
}


fn main() {

    println!("電卓を起動しました");
    println!("終了するにはexitと入力してください");

    loop {
        // 標準入力を待っていることを知らせるために>>を表示

        print!(">> ");
        // 標準出力をフラッシュする フラッシュしないと >> 計算結果になる。
        io::stdout().flush().unwrap();
        // 標準入力から1行読み込む
        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        let s = s.trim();
        if s == "exit" || s == "q" {
            break;
        }

        if s.is_empty() {
            continue;
        }
        let answer = calculate(s.to_string());
        println!("{}", answer);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
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
    fn test_parser() {
        let expression = "3 * 2 + 1 + 5 ";

        let tokens = tokenize(expression.to_string());

        let parser = Parser::new(tokens);

        let ast = parser.parse();

        assert_eq!(ast, Ast::BinOp(
            Box::new(Ast::BinOp(
                Box::new(Ast::BinOp(
                    Box::new(Ast::Number(3)),
                    BinOp::Mul,
                    Box::new(Ast::Number(2)),
                )),
                BinOp::Add,
                Box::new(Ast::Number(1)),
            )),
            BinOp::Add,
            Box::new(Ast::Number(5)),
        ));
    }

    #[test]
    fn test_eval() {
        let expression = "3 + 2 / 2 + 1 + 5 * 2 ";

        let tokens = tokenize(expression.to_string());

        let parser = Parser::new(tokens);

        let ast = parser.parse();

        let val = eval(ast);

        assert_eq!(val, 15);
    }

    #[test]
    fn test_calculate1() {
        let expression = "3 + 2 / 2 + 1 + 5 * 2 ";

        let val = calculate(expression.to_string());

        assert_eq!(val, 15);

    }
    #[test]
    fn test_power() {
        let expression = "3 ** 4 ";
        let val = calculate(expression.to_string());
        assert_eq!(val, 81);

        let expression = "3 ** 4 ** 2 ";
        let val = calculate(expression.to_string());
        assert_eq!(val, 3i32.pow(4i32.pow(2) as u32));
    }

    #[test]
    fn test_calculate2() {
        let expression = " (4 + 5)";
        let val = calculate(expression.to_string());

        assert_eq!(val, 9);

        let expression = " 3 * (4 + 5) / 2 ";

        let val = calculate(expression.to_string());
        assert_eq!(val, 13);

        let expression = "(((3)))";
        let val = calculate(expression.to_string());

        assert_eq!(val, 3);
    }
}