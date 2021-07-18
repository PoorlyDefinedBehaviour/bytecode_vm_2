pub mod chunk;
pub mod compiler;
pub mod disassembler;
pub mod lexer;
pub mod token;
pub mod value;
pub mod vm;

use std::io::{self, Write};

fn main() {
  loop {
    print!("> ");

    io::stdout().flush().expect("flush failed");

    let mut buffer = String::new();

    io::stdin()
      .read_line(&mut buffer)
      .expect("unable to read input");

    match lexer::lex(buffer) {
      Err(errors) => println!("{:?}", errors),
      Ok(tokens) => println!("{:?}", vm::interpret(compiler::compile(tokens))),
    }
  }
}
