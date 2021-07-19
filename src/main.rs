pub mod chunk;
pub mod compiler;
pub mod disassembler;
pub mod lexer;
pub mod token;
pub mod value;
pub mod vm;

use std::io::{self, Write};

use compiler::Compiler;
use vm::{InterpretResult, Vm};

fn main() {
  let mut compiler = Compiler::new();
  let mut vm = Vm::new();

  loop {
    print!("> ");

    io::stdout().flush().expect("flush failed");

    let mut buffer = String::new();

    io::stdin()
      .read_line(&mut buffer)
      .expect("unable to read input");

    match lexer::lex(buffer) {
      Err(errors) => println!("{:?}", errors),
      Ok(tokens) => {
        if let InterpretResult::Ok(Some(result)) = vm.run(compiler.compile(tokens)) {
          println!("{:?}", result);
        }
      }
    }
  }
}
