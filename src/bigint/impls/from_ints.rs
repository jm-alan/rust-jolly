use std::cmp::Ordering;

use crate::{bigint::BigInt, utils::Sign};

impl From<u64> for BigInt {
  #[inline(always)]
  fn from(mut value: u64) -> Self {
    let mut digits = vec![];

    if value > u32::MAX as u64 {
      let mut half = u32::MAX as u64;
      half <<= 32;
      digits.push((value - half) as u32);
      value >>= 32;
    }

    digits.push(value as u32);

    Self {
      sign: match value.cmp(&0) {
        Ordering::Equal => Sign::Zero,
        _ => Sign::Positive,
      },
      digits,
    }
  }
}

impl From<i64> for BigInt {
  #[inline(always)]
  fn from(value: i64) -> Self {
    let mut digits = vec![];

    let sign = match value.cmp(&0) {
      Ordering::Equal => Sign::Zero,
      Ordering::Greater => Sign::Positive,
      Ordering::Less => Sign::Negative,
    };

    let mut unsigned_value = value.unsigned_abs();

    if unsigned_value > u32::MAX as u64 {
      let mut half = u32::MAX as u64;
      half <<= 32;
      digits.push((unsigned_value - half) as u32);
      unsigned_value >>= 32;
    }

    digits.push(unsigned_value as u32);

    Self { sign, digits }
  }
}
