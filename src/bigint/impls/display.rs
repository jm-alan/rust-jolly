use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::bigint::BigInt;

impl Display for BigInt {
  fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
    if self.magnitude() == 1 {
      write!(formatter, "{}", self.digits[0])
    } else {
      // let mut digits_in = self.digits.clone();
      // let mut digits_out = vec![0];
      // while !digits_in.iter().all(|d| d == &0) {
      //   digital_increment(&mut digits_out, IncrementWrap::Ten);
      //   digital_decrement(&mut digits_in, u64::MAX)
      // }
      // digits_out.reverse();
      // let digital_string =
      //   digits_out.iter().map(|d| d.to_string()).collect::<Vec<_>>().join("");
      // write!(formatter, "{}", digital_string)
      todo!();
    }
  }
}
