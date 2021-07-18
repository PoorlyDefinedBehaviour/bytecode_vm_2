use crate::chunk::{Chunk, OpCode};
use crate::value::Value;

use std::collections::VecDeque;

struct Vm {
  chunk: Chunk,
  ip: usize,
  stack: VecDeque<Value>,
}

#[derive(Debug)]
pub enum InterpretResult {
  Ok(Option<Value>),
  CompileError,
  RuntimeError,
}

impl Vm {
  pub fn new(chunk: Chunk) -> Self {
    Vm {
      chunk,
      ip: 0,
      stack: VecDeque::new(),
    }
  }

  pub fn run(&mut self) -> InterpretResult {
    while self.ip < self.chunk.code.len() {
      let instruction = &self.chunk.code[self.ip];

      self.ip += 1;

      match instruction {
        OpCode::Return => {
          return InterpretResult::Ok(self.stack.pop_back());
        }
        OpCode::Constant(constant_index) => {
          let constant = &self.chunk.constants[*constant_index];
          self.stack.push_back(constant.clone());
        }
        OpCode::Negate => match self.stack.pop_back().unwrap() {
          Value::Number(number) => self.stack.push_back(Value::Number(-number)),
          _ => panic!("Operand must be a number"),
        },
        OpCode::Add => {
          let b = self.stack.pop_back().unwrap();
          let a = self.stack.pop_back().unwrap();

          match (a, b) {
            (Value::Number(a), Value::Number(b)) => self.stack.push_back(Value::Number(a + b)),
            _ => panic!("Operands must be numbers"),
          }
        }
        OpCode::Subtract => {
          let b = self.stack.pop_back().unwrap();
          let a = self.stack.pop_back().unwrap();

          match (a, b) {
            (Value::Number(a), Value::Number(b)) => self.stack.push_back(Value::Number(a - b)),
            _ => panic!("Operands must be numbers"),
          }
        }
        OpCode::Multiply => {
          let b = self.stack.pop_back().unwrap();
          let a = self.stack.pop_back().unwrap();

          match (a, b) {
            (Value::Number(a), Value::Number(b)) => self.stack.push_back(Value::Number(a * b)),
            _ => panic!("Operands must be numbers"),
          }
        }
        OpCode::Divide => {
          let b = self.stack.pop_back().unwrap();
          let a = self.stack.pop_back().unwrap();

          match (a, b) {
            (Value::Number(a), Value::Number(b)) => self.stack.push_back(Value::Number(a / b)),
            _ => panic!("Operands must be numbers"),
          }
        }
        OpCode::Nil => self.stack.push_back(Value::Nil),
        OpCode::Boolean(boolean) => self.stack.push_back(Value::Boolean(*boolean)),
      }
    }

    return InterpretResult::Ok(self.stack.pop_back());
  }
}

pub fn interpret(chunk: Chunk) -> InterpretResult {
  let mut vm = Vm::new(chunk);

  vm.run()
}
