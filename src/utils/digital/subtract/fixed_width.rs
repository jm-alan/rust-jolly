use num::{Bounded, FromPrimitive, Integer, Unsigned};
use std::{cmp::Ordering, fmt::Debug};

use super::{digital_iterator_subtract_into_container, ignore_leading_zeroes};
use crate::{
  traits::from_bool::FromBool,
  utils::{digital_cmp, DigitalWrap, Sign},
};

#[inline(always)]
pub fn digital_subtract_fixed_width<I>(
  lhs: &[I],
  rhs: &[I],
  base: DigitalWrap,
) -> ([I; 65536], Sign)
where
  I: Integer + Unsigned + Bounded + FromPrimitive + FromBool + Copy + Debug,
{
  let mut result = [I::zero(); 65536];

  let left_zero = lhs.iter().all(|v| v.is_zero());
  let right_zero = rhs.iter().all(|v| v.is_zero());

  match (left_zero, right_zero) {
    (true, true) => (result, Sign::Zero),
    (true, _) => {
      result.copy_from_slice(rhs);
      (result, Sign::Positive)
    }
    (_, true) => {
      result.copy_from_slice(lhs);

      (result, Sign::Positive)
    }
    _ => {
      let mut larger = lhs;
      let mut smaller = rhs;
      let mut sign = Sign::Positive;

      match digital_cmp(lhs, rhs) {
        Ordering::Equal => return (result, Sign::Zero),
        Ordering::Less => {
          larger = rhs;
          smaller = lhs;
          sign = Sign::Negative;
        }
        _ => {}
      }

      let mut trimmed_larger = ignore_leading_zeroes(larger);
      let mut trimmed_smaller = ignore_leading_zeroes(smaller);

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
