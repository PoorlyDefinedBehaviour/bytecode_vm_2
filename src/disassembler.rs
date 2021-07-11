/// An assembler is an old-school program that akes a file
/// containing human-readable mnemonic names for CPU
/// instructions like ADD and MULT and translates them to their
/// binary machine code equivalent.
/// A dissasembler goes in the othre direction: given a blob
/// of machine code, it spits out a textual listing of their instructions.
use crate::chunk::{Chunk, OpCode};

pub fn disassemble_chunk(chunk: &Chunk) {
  let mut offset = 0;

  while offset < chunk.code.len() {
    offset = disassemble_instruction(chunk, offset);
  }
}

pub fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
  print!("{offset:>0width$} ", offset = offset, width = 4);

  match chunk.code[offset] {
    OpCode::Return => simple_instruction(OpCode::Return, offset),
  }
}

fn simple_instruction(opcode: OpCode, offset: usize) -> usize {
  println!("{:?}", opcode);

  offset + 1
}
