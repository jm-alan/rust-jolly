use crate::{bigint::BigInt, utils::Sign};

impl Default for BigInt {
  fn default() -> Self {
    Self {
      sign: Sign::Zero,
      digits: vec![0],
    }
  }
}
