#[derive(Debug, Clone, Copy)]
pub enum Op {
  Halt = 0,
  Set = 1,
  Push = 2,
  Pop = 3,
  Eq = 4,
  Gt = 5,
  Jmp = 6,
  Jt = 7,
  Jf = 8,
  Add = 9,
  Mult = 10,
  Mod = 11,
  And = 12,
  Or = 13,
  Not = 14,
  Rmem = 15,
  Wmem = 16,
  Call = 17,
  Ret = 18,
  Out = 19,
  In = 20,
  Noop = 21,
}

impl Op {
  pub fn from_code(op_code: usize) -> Option<Op> {
    match op_code {
      0 => Some(Op::Halt),
      1 => Some(Op::Set),
      2 => Some(Op::Push),
      3 => Some(Op::Pop),
      4 => Some(Op::Eq),
      5 => Some(Op::Gt),
      6 => Some(Op::Jmp),
      7 => Some(Op::Jt),
      8 => Some(Op::Jf),
      9 => Some(Op::Add),
      10 => Some(Op::Mult),
      11 => Some(Op::Mod),
      12 => Some(Op::And),
      13 => Some(Op::Or),
      14 => Some(Op::Not),
      15 => Some(Op::Rmem),
      16 => Some(Op::Wmem),
      17 => Some(Op::Call),
      18 => Some(Op::Ret),
      19 => Some(Op::Out),
      20 => Some(Op::In),
      21 => Some(Op::Noop),
      _ => None,
    }
  }
}