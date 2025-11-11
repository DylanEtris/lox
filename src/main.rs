use std::{env::args, io::Error};

use compiler::Lox;

fn main() -> Result<(), Error> {
    let mut compiler = Lox::default();
    let args: Vec<String> = args().collect();
    let args = &args[1..];
    compiler.main(args.to_vec())
}
