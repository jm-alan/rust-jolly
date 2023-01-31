use num::{Bounded, FromPrimitive, Integer, Unsigned};
use std::{cmp::Ordering, fmt::Debug};

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
