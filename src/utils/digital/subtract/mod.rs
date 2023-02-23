mod fixed_width;
mod variable_width;

use num::{Bounded, FromPrimitive, Integer, Unsigned};
use std::{fmt::Debug, iter::Take, slice};

use super::DigitalWrap;

pub use fixed_width::*;
pub use variable_width::*;

#[inline(always)]
fn digital_iterator_subtract_into_container<'iterator_lifetime, I>(
  lhs: &mut impl Iterator<Item = &'iterator_lifetime I>,
  rhs: &mut impl Iterator<Item = &'iterator_lifetime I>,
  result: &mut [I],
  base: DigitalWrap,
) where
  I: 'iterator_lifetime
    + Integer
    + Unsigned
    + Bounded
    + FromPrimitive
    + Copy
    + Debug,
{
  let mut current_idx = 0;
  let mut borrow = false;

  loop {
    match (lhs.next(), rhs.next()) {
      (Some(left), Some(right)) => {
        let after_borrow = wrapping_subtract(
          *left,
          if borrow { I::one() } else { I::zero() },
          base,
        );
        let after_sub = wrapping_subtract(after_borrow, *right, base);

        borrow = &after_borrow > left || after_sub > after_borrow;

        result[current_idx] = after_sub;
      }
      (Some(val), None) => {
        let after_borrow = wrapping_subtract(
          *val,
          if borrow { I::one() } else { I::zero() },
          base,
        );
        result[current_idx] = after_borrow;
        borrow = &after_borrow > val;
      }
      (None, Some(_)) => panic!("Impossible procession of subtraction loop"),
      (None, None) => break,
    }
  }
}

#[inline(always)]
fn ignore_leading_zeroes<I>(digits: &[I]) -> Take<slice::Iter<'_, I>>
where
  I: Integer + Copy + Debug,
{
  if digits.len() == 1 {
    digits.iter().take(1)
  } else {
    let mut end_idx = digits.len() - 1;
    while digits[end_idx] == I::zero() {
      end_idx -= 1;
    }
    digits.iter().take(end_idx + 1)
  }
}

#[inline(always)]
fn wrapping_subtract<I>(lhs: I, rhs: I, base: DigitalWrap) -> I
where
  I: Integer + Unsigned + Bounded + FromPrimitive + Copy + Debug,
{
  let wrap: Option<I> = base.into();

  if rhs > lhs {
    wrap.unwrap() - (rhs - lhs - I::one())
  } else {
    lhs - rhs
  }
}
