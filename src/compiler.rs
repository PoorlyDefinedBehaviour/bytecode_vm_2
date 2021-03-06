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

pub struct Compiler {
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
  pub fn new() -> Self {
    Compiler {
      tokens: Vec::new(),
      position: 0,
      is_in_error_state: false,
      chunk: Chunk::new(),
      prefix_parselets: parselets! {
        &Token::True => Compiler::literal,
        &Token::False => Compiler::literal,
        &Token::Nil => Compiler::literal,
        // TODO: can we get the discriminant without instatiating the variant?
        &Token::Number("any number".to_owned()) => Compiler::literal,
        &Token::Identifier("any identifier".to_owned()) => Compiler::variable
      },
      infix_parselets: parselets! {
        &Token::Plus => Compiler::binary
      },
    }
  }

  fn reset(&mut self) {
    self.position = 0;
    self.is_in_error_state = false;
  }

  fn consume(&mut self, expected_token: &Token) -> Option<(Token, SourceLocation)> {
    let (token, location) = self.tokens[self.position].clone();

    if std::mem::discriminant(&token) != std::mem::discriminant(expected_token) {
      self.error(format!(
        "expected {:?}, got {:?} at line {} and column {}",
        expected_token, token, location.line, location.column
      ));

      None
    } else {
      self.position += 1;
      Some((token, location))
    }
  }

  fn consume_current_token(&mut self) -> (Token, SourceLocation) {
    let (token, location) = self.tokens[self.position].clone();

    self.position += 1;

    (token, location)
  }

  fn error(&mut self, message: String) {
    if self.is_in_error_state {
      return;
    }

    self.is_in_error_state = true;

    println!("{}", message);
  }

  fn synchronize(&mut self) {
    self.is_in_error_state = false;

    loop {
      match self.current_token() {
        Token::Eof
        | Token::Class
        | Token::Function
        | Token::Let
        | Token::For
        | Token::If
        | Token::While
        | Token::Print
        | Token::Return => break,
        _ => self.advance(),
      }
    }
  }

  fn advance(&mut self) {
    self.position += 1;
  }

  fn current_token(&self) -> Token {
    let (token, _location) = &self.tokens[self.position];
    token.clone()
  }

  fn current_token_location(&self) -> SourceLocation {
    let (_token, location) = &self.tokens[self.position];
    location.clone()
  }

  fn parse_precedence(&mut self, precedence: Precedence) {
    match self
      .prefix_parselets
      .get(&std::mem::discriminant(&self.current_token()))
    {
      None => self.error("expected expression".to_owned()),
      Some(prefix_parselet) => {
        prefix_parselet(self);

        while precedence <= token_precedence(&self.current_token()) {
          let infix_parselet = self
            .infix_parselets
            .get(&std::mem::discriminant(&self.current_token()))
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
    let (token, location) = self.consume_current_token();

    match token {
      Token::Number(lexeme) => {
        let value = lexeme.parse::<f64>().unwrap();
        self
          .chunk
          .write_constant(OpCode::Constant, Value::Number(value), location.line);
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
    let (token, location) = self.consume_current_token();

    match token {
      Token::Plus => {
        self.parse_precedence(Precedences::TERM);
        self.chunk.write(OpCode::Add, location.line);
      }
      Token::Minus => {
        self.parse_precedence(Precedences::TERM);
        self.chunk.write(OpCode::Subtract, location.line);
      }
      Token::Slash => {
        self.parse_precedence(Precedences::FACTOR);
        self.chunk.write(OpCode::Divide, location.line);
      }
      Token::Star => {
        self.parse_precedence(Precedences::FACTOR);
        self.chunk.write(OpCode::Multiply, location.line);
      }
      token => panic!("unexpected token {:?}", token),
    }
  }

  fn literal(&mut self) {
    let (token, location) = self.consume_current_token();

    match token {
      Token::False => self.chunk.write(OpCode::Boolean(false), location.line),
      Token::True => self.chunk.write(OpCode::Boolean(true), location.line),
      Token::Nil => self.chunk.write(OpCode::Nil, location.line),
      Token::Number(number) => match number.parse::<f64>() {
        Ok(number) => {
          self
            .chunk
            .write_constant(OpCode::Constant, Value::Number(number), location.line)
        }
        error => panic!("{:?}", error),
      },
      token => panic!("unexpected token {:?}", token),
    }
  }

  fn grouping(&mut self) {
    self.expression();
    self.consume(&Token::RightParen);
  }

  fn print_statement(&mut self) {
    self.consume(&Token::Print);

    self.expression();

    self
      .chunk
      .write(OpCode::Print, self.current_token_location().line)
  }

  fn expression_statement(&mut self) {
    self.expression();

    self
      .chunk
      .write(OpCode::Pop, self.current_token_location().line);
  }

  fn variable(&mut self) {
    let (token, location) = self.consume_current_token();

    match token {
      Token::Identifier(variable_name) => self.chunk.write_constant(
        OpCode::AccessGlobalVariable,
        Value::Identifier(variable_name),
        location.line,
      ),
      token => panic!("unexpected token {:?}", token),
    }
  }

  fn let_declaration(&mut self) {
    self.consume(&Token::Let);

    if let Some((Token::Identifier(identifier), location)) =
      self.consume(&Token::Identifier("any_identifier".to_owned()))
    {
      self.consume(&Token::Assign);

      self.expression();

      self.chunk.write_constant(
        OpCode::DefineGlobalVariable,
        Value::Identifier(identifier),
        location.line,
      )
    }
  }

  pub fn compile(&mut self, tokens: Vec<(Token, SourceLocation)>) -> Chunk {
    self.reset();

    self.tokens = tokens;

    loop {
      if self.is_in_error_state {
        self.synchronize();
      }

      match self.current_token() {
        Token::Eof => return self.chunk.clone(),
        Token::Print => self.print_statement(),
        Token::Let => self.let_declaration(),
        Token::Illegal(character) => panic!("illegal character {:?}", character),
        _ => self.expression_statement(),
      }
    }
  }
}
