use std::{env::args, io::Error};

use compiler::{
    Lox,
    ast_printer::AstPrinter,
    expr::Expr,
    token::{Literal, Token, TokenType},
};

fn main() -> Result<(), Error> {
    //let expression = Expr::Binary {
    //    left: Box::new(Expr::Unary {
    //        operator: Token::new(TokenType::Minus, "-".to_string(), None, 1),
    //        right: Box::new(Expr::Literal {
    //            value: Literal::Number { val: 123.0 },
    //        }),
    //    }),
    //    operator: Token::new(TokenType::Star, "*".to_string(), None, 1),
    //    right: Box::new(Expr::Grouping {
    //        expression: Box::new(Expr::Literal {
    //            value: Literal::Number { val: 45.67 },
    //        }),
    //    }),
    //};
    //let printer = AstPrinter {};
    //println!("{}", printer.print(&expression));
    let mut compiler = Lox::default();
    let args: Vec<String> = args().collect();
    let args = &args[1..];
    compiler.main(args.to_vec())?;
    Ok(())
}
