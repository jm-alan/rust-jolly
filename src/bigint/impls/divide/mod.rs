use std::{
  cmp::Ordering,
  ops::{Div, DivAssign, Rem},
};

use crate::{
  bigint::BigInt,
  utils::{
    digital_cmp, digital_scalar_divide_in_place_u32, digital_scalar_rem_u32,
    Sign,
  },
};

impl Div for &BigInt {
  type Output = BigInt;

  #[inline(always)]
  fn div(self, rhs: Self) -> Self::Output {
    match (self.sign, rhs.sign) {
      (_, Sign::Zero) => panic!("Attempt to divide by zero"),
      (Sign::Zero, _) => BigInt::zero(),
      _ => match self.cmp_magnitude(rhs) {
        Ordering::Greater => BigInt::zero(),
        _ => match digital_cmp(&self.digits, &rhs.digits) {
          Ordering::Less => {
            todo!("Full division not yet implmemented");
          }
          _ => BigInt::zero(),
        },
      },
    }
  }
}

impl DivAssign<u32> for BigInt {
  #[inline(always)]
  fn div_assign(&mut self, rhs: u32) {
    digital_scalar_divide_in_place_u32(&mut self.digits, rhs);
    self.trim_zeroes();
    if self.digits.len() == 1 && self.digits[0] == 0 {
      self.sign = Sign::Zero
    }
  }
}

impl Div<u32> for &BigInt {
  type Output = BigInt;

  #[inline(always)]
  fn div(self, rhs: u32) -> Self::Output {
    let mut cloned = self.clone();

    cloned /= rhs;

    cloned
  }
}

impl Rem<u32> for &BigInt {
  type Output = u32;

  #[inline(always)]
  fn rem(self, rhs: u32) -> Self::Output {
    digital_scalar_rem_u32(&self.digits, rhs)
  }
}
