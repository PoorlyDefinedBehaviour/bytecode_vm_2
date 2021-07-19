#[derive(Debug, PartialEq, Clone)]
pub enum Value {
  Boolean(bool),
  Number(f64),
  Identifier(String),
  Nil,
}
