use std::ops::{Mul, MulAssign};

use crate::{
  bigint::BigInt,
  utils::{
    digital_add_in_place, digital_scalar_multiply_in_place_u32, DigitalWrap,
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
    let mut scaled_mult =
      Vec::with_capacity(self.magnitude() + rhs.magnitude());
    scaled_mult.extend(self.digits.iter().take(self.digits.len() - 1));

    let mut result =
      Vec::with_capacity(usize::max(self.magnitude(), rhs.magnitude()) + 1);
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
  }
}
