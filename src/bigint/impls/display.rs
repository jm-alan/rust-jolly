use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::{
  bigint::BigInt,
  utils::{digital_add_in_place, digital_subtract, Digital, DigitalWrap, Sign},
};

const UINT_MAX_PLUS_ONE: [u64; 2] = [0, 1];

impl Display for BigInt {
  fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
    if self.magnitude() == 1 {
      write!(formatter, "{}", self.digits[0])
    } else {
      let mut digital_copy = self.digits.clone();
      let mut result = digital_copy[0].as_digits();
      digital_copy[0] = 0;

      loop {
        let (difference, sign) =
          digital_subtract(&digital_copy, &UINT_MAX_PLUS_ONE, DigitalWrap::Max);

        digital_add_in_place(
          &mut result,
          &BigInt::u64_max_plus_one(),
          DigitalWrap::Ten,
        );

        if sign == Sign::Zero {
          break;
        }
        digital_copy = difference;
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
