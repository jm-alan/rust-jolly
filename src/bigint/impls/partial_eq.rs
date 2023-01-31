use crate::bigint::BigInt;

impl PartialEq for BigInt {
  #[inline(always)]
  fn eq(&self, other: &Self) -> bool {
    self.sign == other.sign && self.digits == other.digits
  }
}

impl Eq for BigInt {}
