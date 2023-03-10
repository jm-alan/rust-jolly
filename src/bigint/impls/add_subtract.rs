use std::ops::{Add, AddAssign, Sub, SubAssign};

use crate::{
  bigint::BigInt,
  utils::{
    digital_add, digital_add_in_place, digital_subtract, DigitalWrap, Sign,
  },
};

impl BigInt {
  #[inline(always)]
  fn digital_add_assign(&mut self, other: &[u32]) {
    self.sign = Sign::Positive;
    digital_add_in_place(&mut self.digits, other, DigitalWrap::Max);
  }

  #[inline(always)]
  fn digital_subtract_assign(&mut self, other: &[u32]) {
    let (difference, diff_sign) =
      digital_subtract(&self.digits, other, DigitalWrap::Max);

    self.digits = difference;
    if diff_sign.is_negative() {
      self.sign = self.sign.negated();
    } else if diff_sign.is_zero() {
      self.sign = Sign::Zero;
    };
  }

  #[inline(always)]
  fn digital_add(&self, other: &[u32]) -> Self {
    Self {
      sign: Sign::Positive,
      digits: digital_add(&self.digits, other, DigitalWrap::Max),
    }
  }

  #[inline(always)]
  fn digital_subtract(&self, other: &[u32]) -> Self {
    let (digits, diff_sign) =
      digital_subtract(&self.digits, other, DigitalWrap::Max);

    Self {
      sign: match diff_sign {
        Sign::Negative => self.sign.negated(),
        Sign::Zero => Sign::Zero,
        _ => self.sign,
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

  #[inline(always)]
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

impl Add<u32> for &BigInt {
  type Output = BigInt;

  #[inline(always)]
  fn add(self, rhs: u32) -> Self::Output {
    if rhs == 0 {
      self.clone()
    } else {
      self.digital_add(&[rhs])
    }
  }
}

impl AddAssign<u32> for BigInt {
  #[inline(always)]
  fn add_assign(&mut self, rhs: u32) {
    if rhs == 0 {
      return;
    }
    self.digital_add_assign(&[rhs])
  }
}

impl Sub for &BigInt {
  type Output = BigInt;

  #[inline(always)]
  fn sub(self, rhs: Self) -> Self::Output {
    match (self.sign, rhs.sign) {
      (_, Sign::Zero) => self.to_owned(),
      (Sign::Zero, _) => rhs.to_owned(),
      (Sign::Positive, Sign::Positive) | (Sign::Negative, Sign::Negative) => {
        self.digital_subtract(&rhs.digits)
      }
      (Sign::Positive, Sign::Negative) | (Sign::Negative, Sign::Positive) => {
        self.digital_add(&rhs.digits)
      }
    }
  }
}

impl Sub<u32> for &BigInt {
  type Output = BigInt;

  #[inline(always)]
  fn sub(self, rhs: u32) -> Self::Output {
    self.digital_subtract(&[rhs])
  }
}

impl SubAssign<u32> for BigInt {
  #[inline(always)]
  fn sub_assign(&mut self, rhs: u32) {
    self.digital_subtract_assign(&[rhs])
  }
}
