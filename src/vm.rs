use crate::chunk::{Chunk, OpCode};
use crate::value::Value;

use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
pub struct Vm {
  ip: usize,
  stack: VecDeque<Value>,
  globals: HashMap<String, Value>,
}

#[derive(Debug)]
pub enum InterpretResult {
  Ok(Option<Value>),
  CompileError(String),
  RuntimeError(String),
}

impl Vm {
  pub fn new() -> Self {
    Vm {
      ip: 0,
      stack: VecDeque::new(),
      globals: HashMap::new(),
    }
  }

  pub fn run(&mut self, chunk: Chunk) -> InterpretResult {
    dbg!(&chunk);
    while self.ip < chunk.code.len() {
      let instruction = &chunk.code[self.ip];

      self.ip += 1;

      match instruction {
        OpCode::Return => {
          return InterpretResult::Ok(self.stack.pop_back());
        }
        OpCode::Constant(constant_index) => {
          let constant = &chunk.constants[*constant_index];
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
        OpCode::Print => {
          println!("{:?}", self.stack.pop_back().unwrap());
        }
        OpCode::Pop => {
          self.stack.pop_back();
        }
        OpCode::DefineGlobalVariable(global_index) => {
          match chunk.constants[*global_index].clone() {
            Value::Identifier(global_variable_name) => {
              let global_variable_value = self.stack.back().cloned().unwrap();
              self
                .globals
                .insert(global_variable_name, global_variable_value);
              self.stack.pop_back();
            }
            value => panic!("expected global variable name, got {:?}", value),
          }
        }
        OpCode::AccessGlobalVariable(variable_name) => match self.globals.get(variable_name) {
          None => {
            return InterpretResult::RuntimeError(format!("undefined variable {}", variable_name))
          }
          Some(value) => self.stack.push_back(value.clone()),
        },
      }
    }

    return InterpretResult::Ok(self.stack.pop_back());
  }
}
