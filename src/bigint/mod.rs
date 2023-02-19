use std::cmp::Ordering;

use crate::utils::Sign;

pub mod impls;

#[derive(Debug, Clone)]
pub struct BigInt {
  pub(crate) sign: Sign,
  pub(crate) digits: Vec<u32>,
}

const U32_MAX_DIGITS: [u8; 10] = [5, 9, 2, 7, 6, 9, 4, 9, 2, 4];
const U32_MAX_PLUS_ONE_DIGITS: [u8; 10] = [6, 9, 2, 7, 6, 9, 4, 9, 2, 4];
const KARATSUBA_CROSSOVER: usize = 60;

impl BigInt {
  #[inline(always)]
  pub fn zero() -> Self {
    Self::default()
  }

  #[inline(always)]
  pub fn one() -> Self {
    Self {
      sign: Sign::Positive,
      digits: vec![1],
    }
  }

  #[inline(always)]
  pub fn negative_one() -> Self {
    Self {
      sign: Sign::Negative,
      digits: vec![1],
    }
  }

  #[inline(always)]
  pub fn magnitude(&self) -> usize {
    self.digits.len()
  }

  #[inline(always)]
  pub fn trim_zeroes(&mut self) {
    let mut last_nonzero = self.digits.len() - 1;
    while self.digits[last_nonzero] == 0
      && last_nonzero.wrapping_sub(1) != usize::MAX
    {
      last_nonzero = last_nonzero.wrapping_sub(1);
    }
    self.digits.truncate(last_nonzero + 1);
  }

  #[inline(always)]
  pub fn fact(&self) -> Self {
    let mut counter = self.clone();
    let mut result = BigInt::from(1u64);

    while !counter.is_zero() {
      result *= &counter;
      counter -= 1;
    }

    result
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
  pub fn is_zero(&self) -> bool {
    self.sign.is_zero()
  }

  #[inline(always)]
  pub const fn u32_max_digits() -> [u8; 10] {
    U32_MAX_DIGITS
  }

  #[inline(always)]
  pub const fn u32_max_plus_one() -> [u8; 10] {
    U32_MAX_PLUS_ONE_DIGITS
  }

  #[inline(always)]
  pub const fn karatsuba_crossover() -> usize {
    KARATSUBA_CROSSOVER
  }
}
