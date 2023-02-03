use num::{Bounded, FromPrimitive, Integer, Unsigned};
use std::{cmp::Ordering, fmt::Debug};

#[inline(always)]
pub fn digital_cmp<I>(lhs: &[I], rhs: &[I]) -> Ordering
where
  I: Integer + Unsigned + Bounded + FromPrimitive + Copy + Debug,
{
  let mut left_magnitude = lhs.len();
  let mut right_magnitude = rhs.len();

  while left_magnitude > 0 && lhs[left_magnitude - 1].is_zero() {
    left_magnitude -= 1;
  }
  while right_magnitude > 0 && rhs[right_magnitude - 1].is_zero() {
    right_magnitude -= 1;
  }

  match left_magnitude.cmp(&right_magnitude) {
    Ordering::Equal => {
      for current_idx in (0..left_magnitude).rev() {
        match lhs[current_idx].cmp(&rhs[current_idx]) {
          Ordering::Equal => continue,
          ord => return ord,
        }
      }
      Ordering::Equal
    }
    ord => ord,
  }
}
