/// An assembler is an old-school program that takes a file
/// containing human-readable mnemonic names for CPU
/// instructions like ADD and MULT and translates them to their
/// binary machine code equivalent.
/// A dissasembler goes in the other direction: given a blob
/// of machine code, it spits out a textual listing of their instructions.
use crate::chunk::{Chunk, OpCode};
use crate::value::Value;

pub fn disassemble_chunk(chunk: &Chunk) {
  let mut offset = 0;

  while offset < chunk.code.len() {
    offset = disassemble_instruction(chunk, offset);
  }
}

fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
  print!("{offset:>0width$} ", offset = offset, width = 4);

  if offset > 0 && chunk.lines[offset] == chunk.lines[offset - 1] {
    print!("| ");
  } else {
    print!("{} ", chunk.lines[offset]);
  }

  match &chunk.code[offset] {
    OpCode::Constant(index) => {
      indexed_instruction(OpCode::Constant(*index), &chunk.constants[*index], offset)
    }
    OpCode::Return => simple_instruction(OpCode::Return, offset),
    OpCode::Negate => simple_instruction(OpCode::Negate, offset),
    OpCode::Add => simple_instruction(OpCode::Add, offset),
    OpCode::Subtract => simple_instruction(OpCode::Subtract, offset),
    OpCode::Multiply => simple_instruction(OpCode::Multiply, offset),
    OpCode::Divide => simple_instruction(OpCode::Divide, offset),
    OpCode::Nil => simple_instruction(OpCode::Nil, offset),
    OpCode::Boolean(boolean) => simple_instruction(OpCode::Boolean(*boolean), offset),
    OpCode::Print => simple_instruction(OpCode::Print, offset),
    OpCode::Pop => simple_instruction(OpCode::Pop, offset),
    OpCode::DefineGlobalVariable(index) => indexed_instruction(
      OpCode::DefineGlobalVariable(*index),
      &chunk.constants[*index],
      offset,
    ),
    OpCode::AccessGlobalVariable(variable_name) => {
      simple_instruction(OpCode::AccessGlobalVariable(variable_name.clone()), offset)
    }
  }
}

fn indexed_instruction(opcode: OpCode, value: &Value, offset: usize) -> usize {
  println!("{:?} {:?}", opcode, value);

  offset + 1
}

fn simple_instruction(opcode: OpCode, offset: usize) -> usize {
  println!("{:?}", opcode);

  offset + 1
}
