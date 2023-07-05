mod tokenize;
mod parser;
mod eval;

// 四則演算のパーサーをかく
use std::io::{self, Write};
use tokenize::*;
use parser::*;
use eval::*;

fn calculate(s:String) -> i32 {
    let tokens = tokenize(s);
    let parser = Parser::new(tokens);
    let ast = parser.parse();
    eval(ast)
}


fn main() {

    println!("電卓を起動しました");
    println!("終了するにはexitもしくはqと入力してください");

    // english 
    println!("Calculator is started");
    println!("To exit, please input exit or q");

    println!("+, -, *, /, ** are available");

    // example

    println!("example: 3 + 2 ** 3 * 2 + (1 + 2) * 3");
    println!("28");

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
            println!();
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