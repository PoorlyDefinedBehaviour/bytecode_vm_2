use crate::value::Value;

#[derive(Debug, PartialEq)]
pub enum OpCode {
  Constant(usize),
  Negate,
  Return,
  Add,
  Subtract,
  Multiply,
  Divide,
}

#[derive(Debug, PartialEq)]
pub struct Chunk {
  pub code: Vec<OpCode>,
  pub constants: Vec<Value>,
  pub lines: Vec<usize>,
}

impl Chunk {
  pub fn new() -> Self {
    Chunk {
      code: Vec::new(),
      constants: Vec::<Value>::new(),
      lines: Vec::new(),
    }
  }

  pub fn write(&mut self, opcode: OpCode, line: usize) {
    self.code.push(opcode);

    self.lines.push(line);
  }

  pub fn write_constant(&mut self, value: Value, line: usize) {
    self.constants.push(value);

    self.lines.push(line);

    let constant_index = self.constants.len() - 1;

    self.code.push(OpCode::Constant(constant_index));
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn write_adds_opcode_to_chunk_code() {
    let mut chunk = Chunk::new();

    assert_eq!(chunk.code, vec![]);
    assert_eq!(chunk.lines, vec![]);
    assert_eq!(chunk.constants, vec![]);

    chunk.write(OpCode::Return, 1);

    assert_eq!(chunk.code, vec![OpCode::Return]);
    assert_eq!(chunk.lines, vec![1]);
    assert_eq!(chunk.constants, vec![]);

    chunk.write(OpCode::Constant(1), 3);

    assert_eq!(chunk.code, vec![OpCode::Return, OpCode::Constant(1)]);
    assert_eq!(chunk.lines, vec![1, 3]);
    assert_eq!(chunk.constants, vec![]);
  }

  #[test]
  fn write_constant_adds_opcode_to_constants() {
    let mut chunk = Chunk::new();

    assert_eq!(chunk.code, vec![]);
    assert_eq!(chunk.lines, vec![]);
    assert_eq!(chunk.constants, vec![]);

    chunk.write_constant(3.0, 3);

    assert_eq!(chunk.constants, vec![3.0]);

    dbg!(&chunk);
    assert_eq!(chunk.lines, vec![3]);

    chunk.write_constant(5.0, 4);

    assert_eq!(chunk.constants, vec![3.0, 5.0]);

    assert_eq!(chunk.lines, vec![3, 4]);
  }
}
