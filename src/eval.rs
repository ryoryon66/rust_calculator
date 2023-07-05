use crate::parser::{Ast, BinOp};


pub fn eval (ast : Ast) -> i32 {
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


#[cfg(test)]
mod tests {
    use super::*;
    use crate::tokenize::tokenize;
    use crate::parser::Parser;

    #[test]
    fn test_eval() {
        let expression = "3 + 2 / 2 + 1 + 5 * 2 ";

        let tokens = tokenize(expression.to_string());

        let parser = Parser::new(tokens);

        let ast = parser.parse();

        let val = eval(ast);

        assert_eq!(val, 15);
    }

}