pub mod chunk;
pub mod disassembler;
pub mod value;
pub mod vm;

use chunk::{Chunk, OpCode};

fn main() {
  let mut chunk = Chunk::new();

  let constant_index = chunk.write_constant(1.0, 1);
  chunk.write(OpCode::Constant(constant_index), 1);
  chunk.write(OpCode::Return, 1);

  disassembler::disassemble_chunk(&chunk);
}
