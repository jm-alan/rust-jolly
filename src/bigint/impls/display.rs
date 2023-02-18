use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::bigint::BigInt;

impl Display for BigInt {
  #[inline(always)]
  fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
    if self.magnitude() == 1 {
      write!(formatter, "{}", self.digits[0])
    } else {
      let mut cloned = self.clone();

      let mut result = vec![];

      while !cloned.is_zero() {
        result.push(&cloned % 10);
        cloned /= 10;
      }

      let digits_as_string = result
        .iter()
        .rev()
        .map(|digit| digit.to_string())
        .collect::<Vec<_>>()
        .join("");

      write!(formatter, "{digits_as_string}")
    }
  }
}
