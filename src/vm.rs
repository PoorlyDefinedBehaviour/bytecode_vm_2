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
  Ok,
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
    loop {
      let instruction = &self.chunk.code[self.ip];

      self.ip += 1;

      match instruction {
        OpCode::Return => {
          println!("{:?}", self.stack.pop_back());
          return InterpretResult::Ok;
        }
        OpCode::Constant(constant_index) => {
          let constant = self.chunk.constants[*constant_index];
          self.stack.push_back(constant);
        }
        OpCode::Negate => {
          let value = self.stack.pop_back().unwrap();
          self.stack.push_back(-value)
        }
      }
    }
  }
}

pub fn interpret(chunk: Chunk) -> InterpretResult {
  let mut vm = Vm::new(chunk);

  vm.run()
}
