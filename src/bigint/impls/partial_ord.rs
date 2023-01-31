use std::cmp::Ordering;

use crate::{bigint::BigInt, utils::digital_cmp};

impl PartialOrd for BigInt {
  #[inline(always)]
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    match self.sign.partial_cmp(&other.sign) {
      Some(Ordering::Equal) => {
        let self_magnitude = self.digits.len();
        let other_magnitude = other.digits.len();
        match self_magnitude.cmp(&other_magnitude) {
          Ordering::Equal => Some(digital_cmp(&self.digits, &other.digits)),
          ord => Some(ord),
        }
      }
      ord => ord,
    }
  }
}

impl Ord for BigInt {
  #[inline(always)]
  fn cmp(&self, other: &Self) -> Ordering {
    self.partial_cmp(other).unwrap()
  }
}
