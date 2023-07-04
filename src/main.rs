
// 四則演算のパーサーをかく
use std::cell::Cell;

#[derive(Debug,PartialEq,Clone)]
enum Token {
    Number(i32),
    Plus,
    Minus,
    Asterisk,
    Slash,
    LParen,
    RParen,
}

#[derive(Debug)]
enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
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
                tokens.push(Token::Number(buffer.unwrap()));
                tokens.push(Token::Plus);
                buffer = None;
            }

            '-' => {
                tokens.push(Token::Number(buffer.unwrap()));
                tokens.push(Token::Minus);
                buffer = None;
            }

            '*' => {
                tokens.push(Token::Number(buffer.unwrap()));
                tokens.push(Token::Asterisk);
                buffer = None;
            }

            '/' => {
                tokens.push(Token::Number(buffer.unwrap()));
                tokens.push(Token::Slash);
                buffer = None;
            }

            '(' => {
                assert!(buffer.is_none());
                tokens.push(Token::LParen);
            }

            ')' => {
                tokens.push(Token::Number(buffer.unwrap()));
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

        println!("pos: {}, buffer: {}", pos, buffer.unwrap_or(0));
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
        let mut node = self.factor();

        loop {
            if self.consume(Token::Asterisk) {
                let lhs = node;
                let rhs = self.factor();
                node = Ast::BinOp(Box::new(lhs), BinOp::Mul, Box::new(rhs));
            } else if self.consume(Token::Slash) {
                let lhs = node;
                let rhs = self.factor();
                node = Ast::BinOp(Box::new(lhs), BinOp::Div, Box::new(rhs));
            } else {
                break;
            }
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
            }
        }
    }
}


fn main() {
    println!("Hello, world!");
    let expression = "3 * 2 + 1 + 5 ";

    let tokens = tokenize(expression.to_string());

    println!("{:?}", tokens);

    let parser = Parser::new(tokens);

    let ast = parser.parse();

    println!("{:?}", ast);

    let val = eval(ast);

    println!("{}", val);
}