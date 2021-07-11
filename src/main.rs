pub mod chunk;
pub mod disassembler;

use chunk::{Chunk, OpCode};

fn main() {
  let mut chunk = Chunk::new();

  chunk.write(OpCode::Return);

  disassembler::disassemble_chunk(&chunk);
}
