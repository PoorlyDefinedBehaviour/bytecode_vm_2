use crate::token::*;

#[derive(Debug, PartialEq, Clone)]
pub struct LexerError {
  line: usize,
  column: usize,
  message: String,
}

#[derive(Debug)]
struct Lexer {
  source_code: String,
  position: usize,
  next_position: usize,
  line: usize,
  column: usize,
  character: char,
  errors: Vec<LexerError>,
}

impl Lexer {
  pub fn new(source_code: String) -> Lexer {
    let mut lexer = Lexer {
      source_code,
      position: 0,
      next_position: 0,
      character: '\0',
      line: 1,
      column: 0,
      errors: Vec::new(),
    };

    lexer.read_character();

    lexer
  }

  pub fn lex(&mut self) -> Result<Vec<(Token, SourceLocation)>, Vec<LexerError>> {
    let mut tokens = Vec::new();

    while self.has_characters_to_lex() {
      tokens.push(self.next_token());
    }

    if !self.errors.is_empty() {
      return Err(self.errors.clone());
    }

    Ok(tokens)
  }

  fn has_characters_to_lex(&self) -> bool {
    self.position <= self.source_code.len()
  }

  fn read_character(&mut self) {
    if self.next_position >= self.source_code.len() {
      self.character = '\0';
    } else {
      self.character = self.source_code.chars().nth(self.next_position).unwrap();
    }

    if self.character != '\0' {
      self.column += 1;
    }

    if self.character == '\n' {
      self.line += 1;
      self.column = 0;
    }

    self.position = self.next_position;

    self.next_position += 1;
  }

  fn peek_character(&self) -> char {
    if self.next_position >= self.source_code.len() {
      '\0'
    } else {
      self.source_code.chars().nth(self.next_position).unwrap()
    }
  }

  fn skip_whitespace(&mut self) {
    while self.character.is_ascii_whitespace() {
      self.read_character();
    }
  }

  fn error(&mut self, message: String) {
    self.errors.push(LexerError {
      line: self.line,
      column: self.column,
      message,
    });
  }

  fn read_identifier(&mut self) -> String {
    let identifier_starts_at = self.position;

    while self.character.is_alphabetic() {
      self.read_character();
    }

    self
      .source_code
      .chars()
      .skip(identifier_starts_at)
      .take(self.position - identifier_starts_at)
      .collect()
  }

  fn read_number(&mut self) -> String {
    let number_starts_at = self.position;

    while self.character.is_digit(10) {
      self.read_character();
    }

    if self.character == '.' && self.peek_character().is_digit(10) {
      self.read_character();

      while self.character.is_digit(10) {
        self.read_character();
      }
    }

    self
      .source_code
      .chars()
      .skip(number_starts_at)
      .take(self.position - number_starts_at)
      .collect()
  }

  fn read_string(&mut self) -> String {
    let string_starts_at = self.position;

    self.read_character(); // advance past "

    while self.character != '"' && self.has_characters_to_lex() {
      self.read_character();
    }

    let string = self
      .source_code
      .chars()
      .skip(string_starts_at + 1)
      .take(self.position - string_starts_at - 1)
      .collect();

    if self.character != '"' {
      self.read_character(); // advance past "
      self.error(format!(r#"unterminated string: "{}"#, string))
    } else {
      self.read_character(); // advance past "
    }

    string
  }

  fn next_character_is(&self, expected_character: char) -> bool {
    if self.next_position >= self.source_code.len() {
      return false;
    }

    let character = self.source_code.chars().nth(self.next_position).unwrap();

    character == expected_character
  }

  fn source_location(&self) -> SourceLocation {
    // TODO: this is broken, column is always the position of
    // the last character of the current lexeme
    // but it should be position of the the first.
    SourceLocation {
      line: self.line,
      column: self.column,
    }
  }

  fn next_token(&mut self) -> (Token, SourceLocation) {
    self.skip_whitespace();

    let token = match self.character {
      ';' => (Token::Semicolon, self.source_location()),
      '(' => (Token::LeftParen, self.source_location()),
      ')' => (Token::RightParen, self.source_location()),
      ',' => (Token::Comma, self.source_location()),
      '+' => (Token::Plus, self.source_location()),
      '-' => (Token::Minus, self.source_location()),
      '{' => (Token::LeftBrace, self.source_location()),
      '}' => (Token::RightBrace, self.source_location()),
      '[' => (Token::LeftBracket, self.source_location()),
      ']' => (Token::RightBracket, self.source_location()),
      '*' => (Token::Star, self.source_location()),
      '/' => (Token::Slash, self.source_location()),
      '>' => {
        if self.next_character_is('=') {
          self.read_character();
          (Token::GreaterThanOrEqual, self.source_location())
        } else {
          (Token::GreaterThan, self.source_location())
        }
      }
      '<' => {
        if self.next_character_is('=') {
          self.read_character();
          (Token::LessThanOrEqual, self.source_location())
        } else {
          (Token::LessThan, self.source_location())
        }
      }
      '!' => {
        if self.next_character_is('=') {
          self.read_character();
          (Token::NotEqual, self.source_location())
        } else {
          (Token::Bang, self.source_location())
        }
      }
      '=' => {
        if self.next_character_is('=') {
          self.read_character();
          (Token::Equal, self.source_location())
        } else {
          (Token::Assign, self.source_location())
        }
      }
      '\0' => (Token::Eof, self.source_location()),
      '"' => return (Token::String(self.read_string()), self.source_location()),
      character if character.is_alphabetic() => {
        let identifier = self.read_identifier();
        return (lookup_identifier(identifier), self.source_location());
      }
      character if character.is_digit(10) => {
        return (Token::Number(self.read_number()), self.source_location())
      }
      character => (Token::Illegal(character), self.source_location()),
    };

    self.read_character();

    token
  }
}

pub fn lex(source_code: String) -> Result<Vec<(Token, SourceLocation)>, Vec<LexerError>> {
  Lexer::new(source_code).lex()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn keeps_track_of_line_and_column() {
    let test_cases: Vec<(&str, usize, usize)> = vec![
      ("", 1, 0),
      ("abc", 1, 3),
      ("", 1, 0),
      ("a 1 c", 1, 5),
      (
        "let a = 10
let b = 20",
        2,
        10,
      ),
    ];

    for (input, expected_line, expected_column) in test_cases {
      let mut lexer = Lexer::new(String::from(input));

      lexer.lex().ok();

      assert_eq!(lexer.line, expected_line);
      assert_eq!(lexer.column, expected_column);
    }
  }

  #[test]
  fn let_statements() {
    let test_cases: Vec<(&str, Vec<Token>)> = vec![
      (
        "let five = 5;",
        vec![
          Token::Let,
          Token::Identifier(String::from("five")),
          Token::Assign,
          Token::Number(String::from("5")),
          Token::Semicolon,
          Token::Eof,
        ],
      ),
      (
        "let ten = 10;",
        vec![
          Token::Let,
          Token::Identifier(String::from("ten")),
          Token::Assign,
          Token::Number(String::from("10")),
          Token::Semicolon,
          Token::Eof,
        ],
      ),
      (
        "let array = [1, 2, 3]",
        vec![
          Token::Let,
          Token::Identifier(String::from("array")),
          Token::Assign,
          Token::LeftBracket,
          Token::Number(String::from("1")),
          Token::Comma,
          Token::Number(String::from("2")),
          Token::Comma,
          Token::Number(String::from("3")),
          Token::RightBracket,
          Token::Eof,
        ],
      ),
    ];

    for (input, expected_tokens) in test_cases {
      let mut lexer = Lexer::new(String::from(input));

      let tokens = lexer
        .lex()
        .unwrap()
        .iter()
        .map(|(token, _location)| token)
        .cloned()
        .collect::<Vec<Token>>();

      assert_eq!(expected_tokens, tokens);
    }
  }

  #[test]
  fn returns_illegal_token_for_illegal_characters() {
    let test_cases: Vec<(&str, Vec<Token>)> = vec![
      ("let ?", vec![Token::Let, Token::Illegal('?'), Token::Eof]),
      ("@", vec![Token::Illegal('@'), Token::Eof]),
      (
        "@@@",
        vec![
          Token::Illegal('@'),
          Token::Illegal('@'),
          Token::Illegal('@'),
          Token::Eof,
        ],
      ),
    ];

    for (input, expected_tokens) in test_cases {
      let mut lexer = Lexer::new(String::from(input));

      let tokens = lexer
        .lex()
        .unwrap()
        .iter()
        .map(|(token, _location)| token)
        .cloned()
        .collect::<Vec<Token>>();

      assert_eq!(expected_tokens, tokens);
    }
  }

  #[test]
  fn single_character_tokens() {
    let test_cases: Vec<(&str, Vec<Token>)> = vec![
      ("=", vec![Token::Assign, Token::Eof]),
      (";", vec![Token::Semicolon, Token::Eof]),
      ("(", vec![Token::LeftParen, Token::Eof]),
      (")", vec![Token::RightParen, Token::Eof]),
      (",", vec![Token::Comma, Token::Eof]),
      ("+", vec![Token::Plus, Token::Eof]),
      ("-", vec![Token::Minus, Token::Eof]),
      ("!", vec![Token::Bang, Token::Eof]),
      ("{", vec![Token::LeftBrace, Token::Eof]),
      ("}", vec![Token::RightBrace, Token::Eof]),
      ("*", vec![Token::Star, Token::Eof]),
      ("/", vec![Token::Slash, Token::Eof]),
      (">", vec![Token::GreaterThan, Token::Eof]),
      ("<", vec![Token::LessThan, Token::Eof]),
      ("[", vec![Token::LeftBracket, Token::Eof]),
      ("]", vec![Token::RightBracket, Token::Eof]),
    ];

    for (input, expected_tokens) in test_cases {
      let mut lexer = Lexer::new(String::from(input));

      let tokens = lexer
        .lex()
        .unwrap()
        .iter()
        .map(|(token, _location)| token)
        .cloned()
        .collect::<Vec<Token>>();

      assert_eq!(expected_tokens, tokens);
    }
  }

  #[test]
  fn double_character_tokens() {
    let test_cases: Vec<(&str, Vec<Token>)> = vec![
      ("==", vec![Token::Equal, Token::Eof]),
      ("!=", vec![Token::NotEqual, Token::Eof]),
      (">=", vec![Token::GreaterThanOrEqual, Token::Eof]),
      ("<=", vec![Token::LessThanOrEqual, Token::Eof]),
    ];

    for (input, expected_tokens) in test_cases {
      let mut lexer = Lexer::new(String::from(input));

      let tokens = lexer
        .lex()
        .unwrap()
        .iter()
        .map(|(token, _location)| token)
        .cloned()
        .collect::<Vec<Token>>();

      assert_eq!(expected_tokens, tokens);
    }
  }

  #[test]
  fn identifiers() {
    let test_cases: Vec<(&str, Vec<Token>)> = vec![
      (
        "hello",
        vec![Token::Identifier(String::from("hello")), Token::Eof],
      ),
      (
        "foo",
        vec![Token::Identifier(String::from("foo")), Token::Eof],
      ),
      (
        "bar",
        vec![Token::Identifier(String::from("bar")), Token::Eof],
      ),
      ("x", vec![Token::Identifier(String::from("x")), Token::Eof]),
      ("y", vec![Token::Identifier(String::from("y")), Token::Eof]),
    ];

    for (input, expected_tokens) in test_cases {
      let mut lexer = Lexer::new(String::from(input));

      let tokens = lexer
        .lex()
        .unwrap()
        .iter()
        .map(|(token, _location)| token)
        .cloned()
        .collect::<Vec<Token>>();

      assert_eq!(expected_tokens, tokens);
    }
  }

  #[test]
  fn keywords_and_special_values() {
    let test_cases: Vec<(&str, Vec<Token>)> = vec![
      ("return", vec![Token::Return, Token::Eof]),
      ("let", vec![Token::Let, Token::Eof]),
      ("fn", vec![Token::Function, Token::Eof]),
      ("true", vec![Token::True, Token::Eof]),
      ("false", vec![Token::False, Token::Eof]),
      ("if", vec![Token::If, Token::Eof]),
      ("else", vec![Token::Else, Token::Eof]),
      (
        "if(x > 3) {}",
        vec![
          Token::If,
          Token::LeftParen,
          Token::Identifier(String::from("x")),
          Token::GreaterThan,
          Token::Number(String::from("3")),
          Token::RightParen,
          Token::LeftBrace,
          Token::RightBrace,
          Token::Eof,
        ],
      ),
      (
        "if(x > 3) { a } else { b }",
        vec![
          Token::If,
          Token::LeftParen,
          Token::Identifier(String::from("x")),
          Token::GreaterThan,
          Token::Number(String::from("3")),
          Token::RightParen,
          Token::LeftBrace,
          Token::Identifier(String::from("a")),
          Token::RightBrace,
          Token::Else,
          Token::LeftBrace,
          Token::Identifier(String::from("b")),
          Token::RightBrace,
          Token::Eof,
        ],
      ),
    ];

    for (input, expected_tokens) in test_cases {
      let mut lexer = Lexer::new(String::from(input));

      let tokens = lexer
        .lex()
        .unwrap()
        .iter()
        .map(|(token, _location)| token)
        .cloned()
        .collect::<Vec<Token>>();

      assert_eq!(expected_tokens, tokens);
    }
  }

  #[test]
  fn numbers() {
    let test_cases: Vec<(&str, Vec<Token>)> = vec![
      ("10", vec![Token::Number(String::from("10")), Token::Eof]),
      ("0", vec![Token::Number(String::from("0")), Token::Eof]),
      (
        "4124421311",
        vec![Token::Number(String::from("4124421311")), Token::Eof],
      ),
      ("1.0", vec![Token::Number(String::from("1.0")), Token::Eof]),
      ("0.5", vec![Token::Number(String::from("0.5")), Token::Eof]),
      (
        "432342343.43",
        vec![Token::Number(String::from("432342343.43")), Token::Eof],
      ),
      (
        "-0.5",
        vec![Token::Minus, Token::Number(String::from("0.5")), Token::Eof],
      ),
      (
        "-0",
        vec![Token::Minus, Token::Number(String::from("0")), Token::Eof],
      ),
      (
        "-241249129414141241.512521521512",
        vec![
          Token::Minus,
          Token::Number(String::from("241249129414141241.512521521512")),
          Token::Eof,
        ],
      ),
      (
        "-59.42",
        vec![
          Token::Minus,
          Token::Number(String::from("59.42")),
          Token::Eof,
        ],
      ),
    ];

    for (input, expected_tokens) in test_cases {
      let mut lexer = Lexer::new(String::from(input));

      let tokens = lexer
        .lex()
        .unwrap()
        .iter()
        .map(|(token, _location)| token)
        .cloned()
        .collect::<Vec<Token>>();

      assert_eq!(expected_tokens, tokens);
    }
  }

  #[test]
  fn strings() {
    let test_cases: Vec<(&str, Vec<Token>)> = vec![
      (
        r#""10""#,
        vec![Token::String(String::from("10")), Token::Eof],
      ),
      (
        r#""hello world""#,
        vec![Token::String(String::from("hello world")), Token::Eof],
      ),
      (
        r#""-421894124128""#,
        vec![Token::String(String::from("-421894124128")), Token::Eof],
      ),
      (
        r#""let f = fn(x) { f() }""#,
        vec![
          Token::String(String::from("let f = fn(x) { f() }")),
          Token::Eof,
        ],
      ),
    ];

    for (input, expected_tokens) in test_cases {
      let mut lexer = Lexer::new(String::from(input));

      let tokens = lexer
        .lex()
        .unwrap()
        .iter()
        .map(|(token, _location)| token)
        .cloned()
        .collect::<Vec<Token>>();

      assert_eq!(expected_tokens, tokens);
    }
  }

  #[test]
  fn lexer_errors() {
    let test_cases: Vec<(&str, Vec<LexerError>)> = vec![(
      r#""10"#,
      vec![LexerError {
        line: 1,
        column: 3,
        message: String::from(r#"unterminated string: "10"#),
      }],
    )];

    for (input, expected_errors) in test_cases {
      let mut lexer = Lexer::new(String::from(input));

      assert_eq!(Err(expected_errors), lexer.lex());
    }
  }
}
