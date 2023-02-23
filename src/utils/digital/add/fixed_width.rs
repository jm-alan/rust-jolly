use std::fmt::Debug;

use num::{Bounded, FromPrimitive, Integer, Unsigned};

use super::wrapping_add;
use crate::{traits::from_bool::FromBool, utils::DigitalWrap};

#[inline(always)]
pub fn digital_add_fixed_width<I>(
  lhs: &[I],
  rhs: &[I],
  base: DigitalWrap,
) -> [I; 65536]
where
  I: Integer + Unsigned + Bounded + FromPrimitive + FromBool + Copy + Debug,
{
  let mut result = [I::zero(); 65536];

  let one_side_zero = [I::zero()];

  if lhs == one_side_zero {
    result.copy_from_slice(rhs);
    return result;
  }
  if rhs == one_side_zero {
    result.copy_from_slice(lhs);
    return result;
  }

  let mut current_idx = 0;
  let mut carry = false;
  let left_magnitude = lhs.len();
  let right_magnitude = rhs.len();

  while current_idx < left_magnitude && current_idx < right_magnitude {
    let current_left = lhs[current_idx];
    let current_right = rhs[current_idx];
    let current_carry = I::from_bool(carry);
    let before_carry = wrapping_add(current_left, current_right, base);
    let after_carry = wrapping_add(before_carry, current_carry, base);
    carry = before_carry < current_left || after_carry < before_carry;

    result[current_idx] = after_carry;

    current_idx += 1;
  }

  let remainder = if current_idx < left_magnitude {
    lhs
  } else {
    rhs
  };

  let remainder_magnitude = remainder.len();

  while current_idx < remainder_magnitude {
    let current_left = remainder[current_idx];
    let current_carry = I::from_bool(carry);
    let after_carry = wrapping_add(current_left, current_carry, base);

    carry = after_carry < current_left;

    result[current_idx] = after_carry;

    current_idx += 1;
  }

  if carry {
    result[current_idx] = I::one();
  }

  result
}

#[inline(always)]
pub fn digital_add_in_place_fixed_width<I>(
  lhs: &mut [I; 65536],
  rhs: &[I],
  base: DigitalWrap,
) where
  I: Integer + Unsigned + Bounded + FromPrimitive + FromBool + Copy + Debug,
{
  if lhs.iter().all(|el| el == &I::zero()) {
    lhs.copy_from_slice(rhs);
    lhs[rhs.len()..65536].fill(I::zero());
    return;
  }
  if rhs.iter().all(|el| el == &I::zero()) {
    return;
  }

  let mut current_idx = 0;
  let mut carry = false;
  let right_magnitude = rhs.len();

  while current_idx < 65536 && current_idx < right_magnitude {
    let current_left = lhs[current_idx];
    let current_right = rhs[current_idx];
    let current_carry = I::from_bool(carry);
    let before_carry = wrapping_add(current_left, current_right, base);
    let after_carry = wrapping_add(before_carry, current_carry, base);
    carry = before_carry < current_left || after_carry < before_carry;

    lhs[current_idx] = after_carry;

    current_idx += 1;
  }

  while current_idx < right_magnitude {
    let current_left = rhs[current_idx];
    let current_carry = I::from_bool(carry);
    let after_carry = wrapping_add(current_left, current_carry, base);

    carry = after_carry < current_left;

    lhs[current_idx] = after_carry;

    current_idx += 1;
  }

  while carry {
    let current_left = lhs[current_idx];
    let current_carry = I::from_bool(carry);
    let after_carry = wrapping_add(current_left, current_carry, base);

    carry = after_carry < current_left;

    lhs[current_idx] = after_carry;

    current_idx += 1;
  }
}
