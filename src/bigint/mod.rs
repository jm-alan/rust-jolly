use std::cmp::Ordering;

use crate::utils::Sign;

pub mod impls;

#[derive(Debug, Clone)]
pub struct BigInt {
  sign: Sign,
  digits: Vec<u64>,
}

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
}
