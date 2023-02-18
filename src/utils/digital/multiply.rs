use std::ops::Mul;

use super::Sign;

#[inline(always)]
pub fn digital_scalar_multiply_in_place_u32(lhs: &mut Vec<u32>, rhs: u32) {
  let mut carry = 0;

  for el in lhs.iter_mut() {
    let after_fit = fit_shift(higher_order_multiply(*el, rhs), el);
    *el += carry;
    carry = after_fit;
  }

  if carry > 0 {
    lhs.push(carry);
  }
}

#[inline(always)]
fn higher_order_multiply(lhs: u32, rhs: u32) -> u64 {
  lhs as u64 * rhs as u64
}

#[inline(always)]
fn fit_shift(val: u64, target: &mut u32) -> u32 {
  let ceil_val = u32::MAX as u64;
  if val <= ceil_val {
    *target = val as u32;
    0
  } else {
    *target = val as u32;
    (val >> 32) as u32
  }
}

impl Mul for Sign {
  type Output = Self;

  fn mul(self, rhs: Self) -> Self::Output {
    match rhs {
      Sign::Negative => self.negated(),
      Sign::Zero => Sign::Zero,
      _ => self,
    }
  }
}
