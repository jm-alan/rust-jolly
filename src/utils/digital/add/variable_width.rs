use num::{Bounded, FromPrimitive, Integer, Unsigned};
use std::fmt::Debug;

use super::wrapping_add;
use crate::{traits::from_bool::FromBool, utils::DigitalWrap};

#[inline(always)]
pub fn digital_add<I>(lhs: &[I], rhs: &[I], base: DigitalWrap) -> Vec<I>
where
  I: Integer + Unsigned + Bounded + FromPrimitive + FromBool + Copy + Debug,
{
  if lhs == [I::zero()] {
    return rhs.to_vec();
  }
  if rhs == [I::zero()] {
    return lhs.to_vec();
  }

  let mut current_idx = 0;
  let mut carry = false;
  let left_magnitude = lhs.len();
  let right_magnitude = rhs.len();

  let mut result = vec![];

  while current_idx < left_magnitude && current_idx < right_magnitude {
    let current_left = lhs[current_idx];
    let current_right = rhs[current_idx];
    let current_carry = I::from_bool(carry);
    let before_carry = wrapping_add(current_left, current_right, base);
    let after_carry = wrapping_add(before_carry, current_carry, base);
    carry = before_carry < current_left || after_carry < before_carry;

    result.push(after_carry);

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

    result.push(after_carry);

    current_idx += 1;
  }

  if carry {
    result.push(I::one());
  }

  result
}

#[inline(always)]
pub fn digital_add_in_place<I>(lhs: &mut Vec<I>, rhs: &[I], base: DigitalWrap)
where
  I: Integer + Unsigned + Bounded + FromPrimitive + FromBool + Copy + Debug,
{
  if lhs == &[I::zero()] {
    *lhs = rhs.to_vec();
    return;
  }
  if rhs == [I::zero()] {
    return;
  }

  let mut current_idx = 0;
  let mut carry = false;
  let mut left_magnitude = lhs.len();
  let right_magnitude = rhs.len();

  while current_idx < left_magnitude && current_idx < right_magnitude {
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

    lhs.push(after_carry);

    current_idx += 1;
  }

  left_magnitude = lhs.len();

  while carry && current_idx < left_magnitude {
    let current_left = lhs[current_idx];
    let current_carry = I::from_bool(carry);
    let after_carry = wrapping_add(current_left, current_carry, base);

    carry = after_carry < current_left;

    lhs[current_idx] = after_carry;

    current_idx += 1;
  }

  if carry {
    lhs.push(I::one());
  }
}
