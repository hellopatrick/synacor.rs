use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum VMError {
  UnknownOperation(usize),
  EmptyStack,
}

impl fmt::Display for VMError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      VMError::EmptyStack => write!(f, "Stack is empty"),
      VMError::UnknownOperation(op_code) => write!(f, "Unknown Operation: {}", op_code),
    }
  }
}

impl Error for VMError {
  fn description(&self) -> &str {
    match *self {
      VMError::EmptyStack => "empty stack",
      VMError::UnknownOperation(_) => "unknown op code.",
    }
  }
}
