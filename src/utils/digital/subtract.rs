use std::{cmp::Ordering, fmt::Debug, iter::Take, slice};

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
  let mut larger = lhs;
  let mut smaller = rhs;
  let mut sign = Sign::Positive;

  match digital_cmp(lhs, rhs) {
    Ordering::Equal => return (vec![I::zero()], Sign::Zero),
    Ordering::Less => {
      larger = rhs;
      smaller = lhs;
      sign = Sign::Negative;
    }
    _ => {}
  }

  let mut trimmed_larger = ignore_leading_zeroes(larger);
  let mut trimmed_smaller = ignore_leading_zeroes(smaller);

  (
    digital_iterator_subtract(&mut trimmed_larger, &mut trimmed_smaller, base),
    sign,
  )
}

#[inline(always)]
fn digital_iterator_subtract<'iterator_lifetime, I>(
  lhs: &mut impl Iterator<Item = &'iterator_lifetime I>,
  rhs: &mut impl Iterator<Item = &'iterator_lifetime I>,
  base: DigitalWrap,
) -> Vec<I>
where
  I: 'iterator_lifetime
    + Integer
    + Unsigned
    + Bounded
    + FromPrimitive
    + Copy
    + Debug,
{
  let mut result = vec![];
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

        result.push(after_sub);
      }
      (Some(val), None) => {
        let after_borrow = wrapping_subtract(
          *val,
          if borrow { I::one() } else { I::zero() },
          base,
        );
        result.push(after_borrow);
        borrow = &after_borrow > val;
      }
      (None, Some(_)) => panic!("Impossible procession of subtraction loop"),
      (None, None) => break,
    }
  }

  result
}

#[inline(always)]
fn ignore_leading_zeroes<I>(digits: &[I]) -> Take<slice::Iter<'_, I>>
where
  I: Integer + Copy + Debug,
{
  let mut end_idx = digits.len() - 1;
  while digits[end_idx] == I::zero() {
    end_idx -= 1;
  }
  digits.iter().take(end_idx + 1)
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
