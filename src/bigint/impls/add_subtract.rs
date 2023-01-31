use std::ops::{Add, AddAssign};

use crate::{
  bigint::BigInt,
  utils::{digital_add, digital_subtract, DigitalWrap, Sign},
};

impl BigInt {
  #[inline(always)]
  fn digital_add_assign(&mut self, other: &[u64]) {
    self.digits = digital_add(&self.digits, other, DigitalWrap::Max);
  }

  #[inline(always)]
  fn digital_subtract_assign(&mut self, other: &[u64]) {
    let (digits, sign) =
      digital_subtract(&self.digits, other, DigitalWrap::Max);

    self.digits = digits;
    self.sign = if sign == Sign::Negative {
      sign.negate()
    } else {
      sign
    };
  }

  #[inline(always)]
  fn digital_add(&self, other: &[u64]) -> Self {
    Self {
      sign: self.sign,
      digits: digital_add(&self.digits, other, DigitalWrap::Max),
    }
  }

  #[inline(always)]
  fn digital_subtract(&self, other: &[u64]) -> Self {
    let (digits, sign) =
      digital_subtract(&self.digits, other, DigitalWrap::Max);

    Self {
      sign: if sign == Sign::Negative {
        sign.negate()
      } else {
        sign
      },
      digits,
    }
  }

  #[inline(always)]
  pub fn increment(&mut self) {
    self.digital_add_assign(&[1]);
  }

  #[inline(always)]
  pub fn decrement(&mut self) {
    self.digital_subtract_assign(&[1])
  }
}

impl Add for &BigInt {
  type Output = BigInt;

  fn add(self, rhs: Self) -> Self::Output {
    match (self.sign, rhs.sign) {
      (_, Sign::Zero) => self.to_owned(),
      (Sign::Zero, _) => rhs.to_owned(),
      (Sign::Positive, Sign::Positive) | (Sign::Negative, Sign::Negative) => {
        self.digital_add(&rhs.digits)
      }
      (Sign::Positive, Sign::Negative) | (Sign::Negative, Sign::Positive) => {
        self.digital_subtract(&rhs.digits)
      }
    }
  }
}

impl Add<u64> for &BigInt {
  type Output = BigInt;

  fn add(self, rhs: u64) -> Self::Output {
    self.digital_add(&[rhs])
  }
}

impl AddAssign<u64> for BigInt {
  fn add_assign(&mut self, rhs: u64) {
    self.digital_add_assign(&[rhs])
  }
}
