use std::char;
use std::io;
use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};

use crate::errors::VMError;
use crate::ops::Op;

const ADDRESS_SPACE: usize = 32768;

#[derive(PartialEq)]
enum State {
  Running,
  Halted,
}

pub struct VM {
  pointer: usize,
  registers: [usize; 8],
  memory: [usize; ADDRESS_SPACE],
  stack: Vec<usize>,
  buffered_input: Vec<char>,
  state: State,
}

impl VM {
  pub fn new() -> Self {
    VM {
      pointer: 0,
      registers: [0; 8],
      memory: [0; ADDRESS_SPACE],
      stack: Vec::new(),
      buffered_input: Vec::new(),
      state: State::Running,
    }
  }

  pub fn load(reader: &mut Read) -> Self {
    let mut vm = Self::new();
    let mut i = 0;

    while let Ok(m) = reader.read_u16::<LittleEndian>() {
      vm.memory[i] = m as usize;
      i += 1;
    }

    vm
  }

  pub fn run(&mut self) -> Result<(), VMError> {
    loop {
      match self.exec() {
        Ok(_) => {
          if self.state == State::Running {
            continue;
          } else {
            return Ok(());
          }
        }
        err => return err,
      }
    }
  }

  fn exec(&mut self) -> Result<(), VMError> {
    let code = self.fetch();
    let op = Op::from_code(code);

    match op {
      Some(Op::Halt) => self.halt(),
      Some(Op::Set) => self.set(),
      Some(Op::Eq) => self.eq(),
      Some(Op::Gt) => self.gt(),
      Some(Op::Push) => self.push(),
      Some(Op::Pop) => self.pop(),
      Some(Op::Jmp) => self.jmp(),
      Some(Op::Jt) => self.jt(),
      Some(Op::Jf) => self.jf(),
      Some(Op::Add) => self.add(),
      Some(Op::Mult) => self.mult(),
      Some(Op::Mod) => self.modu(),
      Some(Op::And) => self.and(),
      Some(Op::Or) => self.or(),
      Some(Op::Not) => self.not(),
      Some(Op::Call) => self.call(),
      Some(Op::Ret) => self.ret(),
      Some(Op::Rmem) => self.rmem(),
      Some(Op::Wmem) => self.wmem(),
      Some(Op::Out) => self.output(),
      Some(Op::In) => self.input(),
      Some(Op::Noop) => self.noop(),
      None => Err(VMError::UnknownOperation(code)),
    }
  }

  fn fetch(&mut self) -> usize {
    let v = self.memory[self.pointer];

    self.pointer += 1;

    v
  }

  fn read(&mut self) -> usize {
    let v = self.fetch();

    if v < ADDRESS_SPACE {
      v
    } else {
      self.registers[v % ADDRESS_SPACE]
    }
  }

  fn halt(&mut self) -> Result<(), VMError> {
    self.state = State::Halted;

    Ok(())
  }

  fn set(&mut self) -> Result<(), VMError> {
    let a = self.fetch();
    let b = self.read();

    self.registers[a % ADDRESS_SPACE] = b;

    Ok(())
  }

  fn eq(&mut self) -> Result<(), VMError> {
    let a = self.fetch();
    let b = self.read();
    let c = self.read();

    self.registers[a % ADDRESS_SPACE] = if b == c { 1 } else { 0 };

    Ok(())
  }

  fn gt(&mut self) -> Result<(), VMError> {
    let a = self.fetch();
    let b = self.read();
    let c = self.read();

    self.registers[a % ADDRESS_SPACE] = if b > c { 1 } else { 0 };

    Ok(())
  }

  fn push(&mut self) -> Result<(), VMError> {
    let a = self.read();

    self.stack.push(a);

    Ok(())
  }

  fn pop(&mut self) -> Result<(), VMError> {
    let a = self.fetch();

    if let Some(v) = self.stack.pop() {
      self.registers[a % ADDRESS_SPACE] = v;
    } else {
      return Err(VMError::EmptyStack);
    }

    Ok(())
  }

  fn jmp(&mut self) -> Result<(), VMError> {
    let a = self.read();

    self.pointer = a;

    Ok(())
  }

  fn jt(&mut self) -> Result<(), VMError> {
    let a = self.read();
    let b = self.read();

    if a != 0 {
      self.pointer = b
    }

    Ok(())
  }

  fn jf(&mut self) -> Result<(), VMError> {
    let a = self.read();
    let b = self.read();

    if a == 0 {
      self.pointer = b
    }

    Ok(())
  }

  fn add(&mut self) -> Result<(), VMError> {
    let a = self.fetch();
    let b = self.read();
    let c = self.read();

    self.registers[a % ADDRESS_SPACE] = (b + c) % ADDRESS_SPACE;

    Ok(())
  }

  fn mult(&mut self) -> Result<(), VMError> {
    let a = self.fetch();
    let b = self.read();
    let c = self.read();

    self.registers[a % ADDRESS_SPACE] = (b * c) % ADDRESS_SPACE;

    Ok(())
  }

  fn modu(&mut self) -> Result<(), VMError> {
    let a = self.fetch();
    let b = self.read();
    let c = self.read();

    self.registers[a % ADDRESS_SPACE] = (b % c) % ADDRESS_SPACE;

    Ok(())
  }

  fn and(&mut self) -> Result<(), VMError> {
    let a = self.fetch();
    let b = self.read();
    let c = self.read();

    self.registers[a % ADDRESS_SPACE] = (b & c) % ADDRESS_SPACE;

    Ok(())
  }

  fn or(&mut self) -> Result<(), VMError> {
    let a = self.fetch();
    let b = self.read();
    let c = self.read();

    self.registers[a % ADDRESS_SPACE] = (b | c) % ADDRESS_SPACE;

    Ok(())
  }

  fn not(&mut self) -> Result<(), VMError> {
    let a = self.fetch();
    let b = self.read();

    self.registers[a % ADDRESS_SPACE] = !b % ADDRESS_SPACE;

    Ok(())
  }

  fn call(&mut self) -> Result<(), VMError> {
    let a = self.read();

    self.stack.push(self.pointer);

    self.pointer = a;

    Ok(())
  }

  fn ret(&mut self) -> Result<(), VMError> {
    if let Some(r) = self.stack.pop() {
      self.pointer = r;
    } else {
      return Err(VMError::EmptyStack);
    }

    Ok(())
  }

  fn rmem(&mut self) -> Result<(), VMError> {
    let a = self.fetch();
    let b = self.read();

    self.registers[a % ADDRESS_SPACE] = self.memory[b];
    Ok(())
  }

  fn wmem(&mut self) -> Result<(), VMError> {
    let a = self.read();
    let b = self.read();

    self.memory[a] = b;

    Ok(())
  }

  fn output(&mut self) -> Result<(), VMError> {
    let a = self.read();

    if let Some(c) = char::from_u32(a as u32) {
      print!("{}", c);
    } else {
      panic!("Unknown char.")
    }

    Ok(())
  }

  fn input(&mut self) -> Result<(), VMError> {
    if self.buffered_input.is_empty() {
      let reader = io::stdin();
      let mut input = String::new();

      reader.read_line(&mut input).expect("Failed to read line");

      match input.as_str() {
        // implement custom commands here.
        _ => self.buffered_input = input.chars().rev().collect(),
      };

      self.buffered_input = input.chars().rev().collect();
    }

    let a = self.fetch();

    self.registers[a % ADDRESS_SPACE] = self.buffered_input.pop().unwrap_or_default() as usize;

    Ok(())
  }

  fn noop(&mut self) -> Result<(), VMError> {
    Ok(())
  }
}
