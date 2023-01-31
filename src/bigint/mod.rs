use std::cmp::Ordering;

use crate::utils::Sign;

pub mod impls;

#[derive(Debug, Clone)]
pub struct BigInt {
  sign: Sign,
  digits: Vec<u64>,
}

const U64_MAX_DIGITS: [u8; 20] =
  [5, 1, 6, 1, 5, 5, 9, 0, 7, 3, 7, 0, 4, 4, 7, 6, 4, 4, 8, 1];

impl BigInt {
  pub fn new(init: i64) -> Self {
    Self {
      sign: match init.cmp(&0) {
        Ordering::Equal => Sign::Zero,
        Ordering::Greater => Sign::Positive,
        Ordering::Less => Sign::Negative,
      },
      digits: vec![init.unsigned_abs()],
    }
  }

  pub fn zero() -> Self {
    Self::default()
  }

  #[inline(always)]
  pub fn magnitude(&self) -> usize {
    self.digits.len()
  }

  #[inline(always)]
  pub fn cmp_magnitude(&self, other: &Self) -> Ordering {
    usize::cmp(&self.magnitude(), &other.magnitude())
  }

  #[inline(always)]
  pub fn negate(&mut self) {
    use Sign::{Negative, Positive, Zero};
    self.sign = match self.sign {
      Positive => Negative,
      Negative => Positive,
      Zero => Zero,
    }
  }

  #[inline(always)]
  pub fn zero_out(&mut self) {
    self.sign = Sign::Zero;
    self.digits = vec![0];
  }

  #[inline(always)]
  const fn u64_max_digits() -> [u8; 20] {
    U64_MAX_DIGITS
  }
}
