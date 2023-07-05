use crate::tokenize::*;
use std::cell::Cell;

#[derive(Debug,PartialEq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Power,
}

#[derive(Debug,PartialEq)]
pub enum Ast {
    Number(i32),
    BinOp(Box<Ast>, BinOp, Box<Ast>),
}

pub struct Parser {
    tokens: Vec<Token>,
    pos : Cell<usize>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
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

    pub fn parse(&self) -> Ast {
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



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser(){
        let expression = "1 + 2";
        let tokens = tokenize(expression.to_string());
        let parser = Parser::new(tokens);
        let ast = parser.parse();

        assert_eq!(ast, 
            Ast::BinOp(
            Box::new(Ast::Number(1)),
            BinOp::Add,
            Box::new(Ast::Number(2)),
        ));


        }

    #[test]
    fn test_parser2() {
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


}