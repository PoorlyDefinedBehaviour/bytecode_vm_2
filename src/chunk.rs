#[derive(Debug, PartialEq)]
pub enum OpCode {
  Return,
}

#[derive(Debug, PartialEq)]
pub struct Chunk {
  pub code: Vec<OpCode>,
}

impl Chunk {
  pub fn new() -> Self {
    Chunk { code: Vec::new() }
  }

  pub fn write(&mut self, opcode: OpCode) {
    self.code.push(opcode);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn write_adds_opcode_to_chunk_code() {
    let mut chunk = Chunk::new();

    assert_eq!(chunk.code, vec![]);

    chunk.write(OpCode::Return);

    assert_eq!(chunk.code, vec![OpCode::Return]);
  }
}
