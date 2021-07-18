use crate::chunk::{Chunk, OpCode};
use crate::token::{SourceLocation, Token};
use crate::value::Value;

use std::collections::HashMap;

#[non_exhaustive]
struct Precedences;

type Precedence = i32;

impl Precedences {
  pub const NONE: Precedence = 1;
  pub const ASSIGNMENT: Precedence = 2; // =
  pub const OR: Precedence = 3; // or
  pub const AND: Precedence = 4; // and
  pub const EQUALITY: Precedence = 5; // == !=
  pub const COMPARISON: Precedence = 6; // < > <= >=
  pub const TERM: Precedence = 7; // + -
  pub const FACTOR: Precedence = 8; // * /
  pub const UNARY: Precedence = 9; // ! -
  pub const CALL: Precedence = 10; // . ()
  pub const PRIMARY: Precedence = 11;
}

fn token_precedence(token: &Token) -> Precedence {
  use Token::*;

  match token {
    Assign => Precedences::ASSIGNMENT,
    Or => Precedences::OR,
    And => Precedences::AND,
    Equal | NotEqual => Precedences::EQUALITY,
    GreaterThan | LessThan | GreaterThanOrEqual | LessThanOrEqual => Precedences::COMPARISON,
    Plus | Minus => Precedences::TERM,
    Star | Slash => Precedences::FACTOR,
    Dot => Precedences::CALL,
    _ => Precedences::NONE,
  }
}

type Parselet = fn(&mut Compiler);

struct Compiler {
  tokens: Vec<(Token, SourceLocation)>,
  position: usize,
  is_in_error_state: bool,
  chunk: Chunk,
  prefix_parselets: HashMap<std::mem::Discriminant<Token>, Parselet>,
  infix_parselets: HashMap<std::mem::Discriminant<Token>, Parselet>,
}

macro_rules! parselets {
    ($($key: expr => $value: expr), *) => {{
      let mut map: HashMap<std::mem::Discriminant<Token>, Parselet> = HashMap::new();
      $(
        let key = std::mem::discriminant($key);
        map.insert(key, $value);
      )*
      map
    }};
}

impl Compiler {
  fn new(tokens: Vec<(Token, SourceLocation)>) -> Self {
    let mut compiler = Compiler {
      tokens,
      position: 0,
      is_in_error_state: false,
      chunk: Chunk::new(),
      prefix_parselets: parselets! {
        &Token::True => Compiler::literal,
        &Token::False => Compiler::literal,
        &Token::Nil => Compiler::literal
      },
      infix_parselets: parselets! {},
    };

    compiler
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

  fn parse_precedence(&mut self, precedence: Precedence) {
    //self.advance();

    let (token, _) = self.tokens[self.position].clone();

    match self.prefix_parselets.get(&std::mem::discriminant(&token)) {
      None => self.error("expected expression".to_owned()),
      Some(prefix_parselet) => {
        prefix_parselet(self);

        while precedence <= token_precedence(&token) {
          //self.advance();

          let infix_parselet = self
            .infix_parselets
            .get(&std::mem::discriminant(&token))
            .unwrap();

          infix_parselet(self);
        }
      }
    }
  }

  fn expression(&mut self) {
    self.parse_precedence(Precedences::ASSIGNMENT);
  }

  fn number(&mut self) {
    let (token, location) = &self.tokens[self.position];

    match token {
      Token::Number(lexeme) => {
        let value = lexeme.parse::<f64>().unwrap();
        self
          .chunk
          .write_constant(Value::Number(value), location.line);
      }
      token => panic!("expected number got {:?}", token),
    }
  }

  fn unary(&mut self) {
    let (token, location) = self.tokens[self.position].clone();

    self.parse_precedence(Precedences::UNARY);

    match token {
      Token::Minus => self.chunk.write(OpCode::Negate, location.line),
      token => panic!("unhandled token {:?}", token),
    }
  }

  fn binary(&mut self) {
    let (token, location) = self.tokens[self.position].clone();
  }

  fn literal(&mut self) {
    let (token, location) = &self.tokens[self.position];

    match token {
      Token::False => self.chunk.write(OpCode::Boolean(false), location.line),
      Token::True => self.chunk.write(OpCode::Boolean(true), location.line),
      Token::Nil => self.chunk.write(OpCode::Nil, location.line),
      token => panic!("unexpected token {:?}", token),
    }
  }

  fn grouping(&mut self) {
    self.expression();
    self.consume(&Token::RightParen);
  }
}

pub fn compile(tokens: Vec<(Token, SourceLocation)>) -> Chunk {
  let mut compiler = Compiler::new(tokens);

  compiler.expression();

  compiler.chunk
}
