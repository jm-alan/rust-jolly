use std::{cmp::Ordering, fmt::Debug};

use num::{Bounded, FromPrimitive, Integer, Unsigned};

use super::{digital_cmp, DigitalWrap, Sign};

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

  (trim_leading_zeroes(&result), sign)
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
