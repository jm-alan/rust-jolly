mod partial_ord;
mod sign;

use num::{Bounded, FromPrimitive, Integer, Unsigned};
use std::{cmp::Ordering, fmt::Debug};

pub use self::sign::Sign;

#[derive(Debug, Default, Clone, Copy)]
pub enum DigitalWrap {
  #[default]
  Max,
  Ten,
  U128(u128),
  U64(u64),
  U32(u32),
  U16(u16),
  U8(u8),
}

impl ToString for DigitalWrap {
  fn to_string(&self) -> String {
    match self {
      DigitalWrap::Max => "Maximum Value".to_string(),
      DigitalWrap::Ten => "10".to_string(),
      DigitalWrap::U128(val) => format!("{val}u128"),
      DigitalWrap::U64(val) => format!("{val}u64"),
      DigitalWrap::U32(val) => format!("{val}u32"),
      DigitalWrap::U16(val) => format!("{val}u16"),
      DigitalWrap::U8(val) => format!("{val}u8"),
    }
  }
}

#[inline(always)]
pub fn digital_cmp<I>(lhs: &[I], rhs: &[I]) -> Ordering
where
  I: Integer + Unsigned + Bounded + FromPrimitive + Copy + Debug,
{
  match lhs.len().cmp(&rhs.len()) {
    Ordering::Equal => {
      for (idx, el) in lhs.iter().enumerate().rev() {
        match el.cmp(&rhs[idx]) {
          Ordering::Equal => continue,
          ord => return ord,
        }
      }
      Ordering::Equal
    }
    ord => ord,
  }
}

// [8, 3, 5, 0, 6, 2, 8, 1, 0, 7, 1, 4] +
// [0, 5, 2, 0, 4, 7, 5, 3, 1, 0, 1, 5]
// [8, 8, 7, 0, 0, 0, 4, 5, 1, 7, 2, 9]

#[inline(always)]
pub fn digital_add<I>(lhs: &[I], rhs: &[I], base: DigitalWrap) -> Vec<I>
where
  I: Integer + Unsigned + Bounded + FromPrimitive + Copy + Debug,
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
    let current_carry = if carry { I::one() } else { I::zero() };
    let before_carry = wrapping_add(current_left, current_right, base);
    let after_carry = wrapping_add(before_carry, current_carry, base);
    carry = before_carry < current_left || after_carry < before_carry;

    result.push(after_carry);

    current_idx += 1;
  }

  exhaust_addition_remainder(&mut result, lhs, current_idx, &mut carry, base);
  exhaust_addition_remainder(&mut result, rhs, current_idx, &mut carry, base);

  result
}

#[inline(always)]
pub fn digital_subtract<I>(
  lhs: &[I],
  rhs: &[I],
  base: DigitalWrap,
) -> (Vec<I>, Sign)
where
  I: Integer + Unsigned + Bounded + FromPrimitive + Copy + Debug,
{
  let trimmed_left = trim_leading_zeroes(lhs);
  let trimmed_right = trim_leading_zeroes(rhs);
  let mut sign = Sign::Positive;
  let mut larger = &trimmed_left;
  let mut smaller = &trimmed_right;

  match digital_cmp(lhs, rhs) {
    Ordering::Equal => return (vec![I::zero()], Sign::Zero),
    Ordering::Less => {
      larger = &trimmed_right;
      smaller = &trimmed_left;
      sign = Sign::Negative;
    }
    _ => {}
  }

  let mut current_idx = 0;
  let mut borrow = false;
  let larger_magnitude = larger.len();
  let smaller_magnitude = smaller.len();
  let mut result = vec![];

  while current_idx < smaller_magnitude {
    let current_larger = larger[current_idx];
    let current_smaller = smaller[current_idx];
    let current_borrow = if borrow { I::one() } else { I::zero() };
    let mut current_result =
      wrapping_subtract(current_larger, current_smaller, base);

    borrow = current_result > current_larger;

    current_result = wrapping_subtract(current_result, current_borrow, base);

    result.push(current_result);

    current_idx += 1;
  }

  while current_idx < larger_magnitude {
    let current_larger = larger[current_idx];
    let current_borrow = if borrow { I::one() } else { I::zero() };
    let current_result =
      wrapping_subtract(current_larger, current_borrow, base);

    borrow = current_result >= current_larger;

    result.push(current_result);

    current_idx += 1;
  }

  (result, sign)
}

#[inline(always)]
fn exhaust_addition_remainder<I>(
  result: &mut Vec<I>,
  remaining_digits: &[I],
  mut current_idx: usize,
  carry: &mut bool,
  base: DigitalWrap,
) where
  I: Integer + Unsigned + Bounded + FromPrimitive + Copy + Debug,
{
  let remainder_magnitude = remaining_digits.len();
  while current_idx < remainder_magnitude {
    let current_val = remaining_digits[current_idx];
    let current_carry = if *carry { I::one() } else { I::zero() };
    let current_result = wrapping_add(current_val, current_carry, base);
    *carry = current_result <= current_val;
    result.push(current_result);
    current_idx += 1;
  }

  if *carry {
    result.push(I::one());
  }

  *carry = false;
}

#[inline(always)]
fn trim_leading_zeroes<I>(digits: &[I]) -> Vec<I>
where
  I: Integer + Unsigned + FromPrimitive + Copy + Debug,
{
  let mut end_idx = digits.len() - 1;
  while digits[end_idx] == I::zero() {
    end_idx -= 1;
  }
  digits.iter().take(end_idx + 1).copied().collect()
}

#[inline(always)]
fn wrapping_add<I>(lhs: I, rhs: I, base: DigitalWrap) -> I
where
  I: Integer + Unsigned + Bounded + FromPrimitive + Copy + Debug,
{
  let option_wrap: Option<I> = base.into();
  let Some(wrap_point) = option_wrap else {
    panic!("Failed to create digital wrap point from supplied base");
  };

  let ceil = wrap_point - lhs;
  if rhs > ceil {
    rhs - ceil - I::one()
  } else {
    lhs + rhs
  }
}

#[inline(always)]
fn wrapping_subtract<I>(lhs: I, rhs: I, base: DigitalWrap) -> I
where
  I: Integer + Unsigned + Bounded + FromPrimitive + Copy + Debug,
{
  let option_wrap: Option<I> = base.into();
  let Some(wrap_point) = option_wrap else {
    panic!("Failed to create digital wrap point from supplied base");
  };

  if rhs > lhs {
    wrap_point - (rhs - lhs - I::one())
  } else {
    lhs - rhs
  }
}

impl<I> From<DigitalWrap> for Option<I>
where
  I: Integer + Unsigned + Bounded + FromPrimitive + Copy + Debug,
{
  fn from(base: DigitalWrap) -> Self {
    match base {
      DigitalWrap::Max => Some(I::max_value()),
      DigitalWrap::Ten => I::from_u8(9),
      DigitalWrap::U128(wrap) => I::from_u128(wrap - 1),
      DigitalWrap::U64(wrap) => I::from_u64(wrap - 1),
      DigitalWrap::U32(wrap) => I::from_u32(wrap - 1),
      DigitalWrap::U16(wrap) => I::from_u16(wrap - 1),
      DigitalWrap::U8(wrap) => I::from_u8(wrap - 1),
    }
  }
}
