use std::ops::{Mul, MulAssign};

use crate::{
  bigint::BigInt,
  utils::{digital_scalar_multiply_in_place_u32, Sign},
};

impl Mul<u32> for &BigInt {
  type Output = BigInt;

  #[inline(always)]
  fn mul(self, rhs: u32) -> Self::Output {
    if rhs == 0 || self.is_zero() {
      BigInt::zero()
    } else {
      let mut result = self.clone();
      result.mul_assign(rhs);
      result
    }
  }
}

impl MulAssign<u32> for BigInt {
  #[inline(always)]
  fn mul_assign(&mut self, rhs: u32) {
    if rhs == 0 {
      self.zero_out()
    } else if self.sign != Sign::Zero {
      digital_scalar_multiply_in_place_u32(&mut self.digits, rhs);
    }
  }
}
