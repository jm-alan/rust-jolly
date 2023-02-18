use std::ops::{Mul, MulAssign};

use crate::{
  bigint::BigInt,
  utils::{
    digital_add_in_place, digital_scalar_multiply_in_place_u32, DigitalWrap,
    Sign,
  },
};

impl Mul for &BigInt {
  type Output = BigInt;

  #[inline(always)]
  fn mul(self, rhs: &BigInt) -> Self::Output {
    let mut result = self.clone();

    result *= rhs;

    result
  }
}

impl MulAssign<&BigInt> for BigInt {
  #[inline(always)]
  fn mul_assign(&mut self, rhs: &BigInt) {
    if rhs.is_zero() {
      return self.zero_out();
    }

    let result_magnitude = self.magnitude() + rhs.magnitude();

    let mut scaled_mult = Vec::with_capacity(result_magnitude);
    scaled_mult.extend(self.digits.iter().take(self.digits.len() - 1));

    let mut result = Vec::with_capacity(result_magnitude);
    result.push(0);

    for (idx, digit) in rhs.digits.iter().enumerate() {
      for num in scaled_mult.iter_mut().take(idx) {
        *num = 0;
      }

      scaled_mult[idx..(idx + self.digits.len() - 1)]
        .copy_from_slice(&self.digits[0..(self.digits.len() - 1)]);
      scaled_mult.push(self.digits[self.digits.len() - 1]);

      digital_scalar_multiply_in_place_u32(&mut scaled_mult, *digit);

      digital_add_in_place(&mut result, &scaled_mult, DigitalWrap::Max);
    }

    self.digits = result;
    if rhs.sign == Sign::Negative {
      self.sign = self.sign.negated();
    }
  }
}

impl BigInt {
  #[inline(always)]
  pub fn pow(&self, rhs: u64) -> Self {
    let mut result = self.clone();

    result.pow_assign(rhs);

    result
  }

  #[inline(always)]
  pub fn pow_assign(&mut self, rhs: u64) {
    for _ in 1..rhs {
      *self *= &rhs.into();
    }
  }
}
