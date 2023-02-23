use num::{Bounded, FromPrimitive, Integer, Unsigned};
use std::{cmp::Ordering, fmt::Debug};

use super::{digital_iterator_subtract_into_container, ignore_leading_zeroes};
use crate::utils::{digital_cmp, DigitalWrap, Sign};

#[inline(always)]
pub fn digital_subtract<I>(
  lhs: &[I],
  rhs: &[I],
  base: DigitalWrap,
) -> (Vec<I>, Sign)
where
  I: Integer + Unsigned + Bounded + FromPrimitive + Copy + Debug,
{
  let left_zero = lhs.iter().all(|v| v == &I::zero());
  let right_zero = rhs.iter().all(|v| v == &I::zero());
  match (left_zero, right_zero) {
    (true, true) => (vec![I::zero()], Sign::Zero),
    (true, _) => (rhs.to_vec(), Sign::Positive),
    (_, true) => (lhs.to_vec(), Sign::Positive),
    _ => {
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

      let mut result = vec![];

      digital_iterator_subtract_into_container(
        &mut trimmed_larger,
        &mut trimmed_smaller,
        &mut result,
        base,
      );
      (result, sign)
    }
  }
}
