use std::ops::Mul;

use super::{digital_add, wrapping_add, DigitalWrap, Sign};
use crate::{bigint::BigInt, utils::digital_add_in_place};

#[inline(always)]
pub fn digital_multiply_u32(lhs: &[u32], rhs: &[u32]) -> Vec<u32> {
  match (lhs.len(), rhs.len()) {
    (0, _) => rhs.to_owned(),
    (_, 0) => lhs.to_owned(),
    (1, 1) => {
      let high_res = higher_order_multiply(lhs[0], rhs[0]);
      if high_res > u32::MAX as u64 {
        vec![high_res as u32, (high_res >> 32) as u32]
      } else {
        vec![high_res as u32]
      }
    }
    (1, _) => digital_scalar_multiply_u32(rhs, lhs[0]),
    (_, 1) => digital_scalar_multiply_u32(lhs, rhs[0]),
    (left_magnitude, right_magnitude) => {
      let result_magnitude = left_magnitude + right_magnitude;

      let standing_zeroes = vec![0; result_magnitude];
      let mut scaled_mult = vec![0; result_magnitude];
      let mut result = vec![0; result_magnitude];

      for (idx, digit) in rhs.iter().enumerate() {
        scaled_mult.copy_from_slice(&standing_zeroes);

        scaled_mult[idx..(idx + left_magnitude)]
          .copy_from_slice(&lhs[0..left_magnitude]);

        digital_scalar_multiply_in_place_u32(&mut scaled_mult, *digit);

        digital_add_in_place(&mut result, &scaled_mult, DigitalWrap::Max);
      }

      result
    }
  }
}

#[inline(always)]
pub fn digital_scalar_multiply_in_place_u32(lhs: &mut Vec<u32>, rhs: u32) {
  let mut carry = 0;

  for el in lhs.iter_mut() {
    let after_fit = fit_shift(higher_order_multiply(*el, rhs), el);

    let before_carry = *el;

    *el = wrapping_add(*el, carry, DigitalWrap::Max);

    carry = after_fit + (*el < before_carry) as u32;
  }

  if carry > 0 {
    lhs.push(carry);
  }
}

#[inline(always)]
pub fn digital_scalar_multiply_u32(lhs: &[u32], rhs: u32) -> Vec<u32> {
  let mut clone = lhs.to_vec();
  digital_scalar_multiply_in_place_u32(&mut clone, rhs);
  clone
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
