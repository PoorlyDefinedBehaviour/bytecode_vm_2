pub mod chunk;
pub mod disassembler;
pub mod lexer;
pub mod token;
pub mod value;
pub mod vm;

use chunk::{Chunk, OpCode};
use std::io::{self, Write};

fn main() {
  loop {
    print!("> ");

    io::stdout().flush().expect("flush failed");

    let mut buffer = String::new();

    io::stdin()
      .read_line(&mut buffer)
      .expect("unable to read input");
  }
}
