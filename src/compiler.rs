use crate::chunk::{Chunk, OpCode};
use crate::token::{SourceLocation, Token};

struct Compiler {
  tokens: Vec<(Token, SourceLocation)>,
  position: usize,
  is_in_error_state: bool,
  chunk: Chunk,
}

impl Compiler {
  fn new(tokens: Vec<(Token, SourceLocation)>) -> Self {
    Compiler {
      tokens,
      position: 0,
      is_in_error_state: false,
      chunk: Chunk::new(),
    }
  }

  fn consume(&mut self, expected_token: &Token) {
    let (token, location) = &self.tokens[self.position];

    if token == expected_token {
      self.position += 1;
      return;
    }

    self.error(format!(
      "expected {:?}, got {:?} at line {} and column {}",
      expected_token, token, location.line, location.column
    ));
  }

  fn error(&mut self, message: String) {
    if self.is_in_error_state {
      return;
    }

    self.is_in_error_state = true;

    println!("{}", message);
  }

  fn expression(&mut self) {}

  fn number(&mut self) {
    let (token, location) = &self.tokens[self.position];

    match token {
      Token::Number(lexeme) => {
        let value = lexeme.parse::<f64>().unwrap();
        self.chunk.write_constant(value, location.line);
      }
      token => panic!("expected number got {:?}", token),
    }
  }

  fn grouping(&mut self) {
    self.expression();
    self.consume(&Token::RightParen);
  }
}

pub fn compile(tokens: Vec<(Token, SourceLocation)>) {
  let mut compiler = Compiler::new(tokens);
}
