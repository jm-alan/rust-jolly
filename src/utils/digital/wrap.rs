use std::fmt::Debug;

use num::{Bounded, FromPrimitive, Integer, Unsigned};

#[derive(Debug, Default, Clone, Copy)]
pub enum DigitalWrap {
  #[default]
  Max,
  Ten,
  U128(u128),
  U64(u64),
  U32(u32),
  U16(u16),
  U8(u8),
}

impl ToString for DigitalWrap {
  fn to_string(&self) -> String {
    match self {
      DigitalWrap::Max => "Maximum Value".to_string(),
      DigitalWrap::Ten => "10".to_string(),
      DigitalWrap::U128(val) => format!("{val}u128"),
      DigitalWrap::U64(val) => format!("{val}u64"),
      DigitalWrap::U32(val) => format!("{val}u32"),
      DigitalWrap::U16(val) => format!("{val}u16"),
      DigitalWrap::U8(val) => format!("{val}u8"),
    }
  }
}

impl<I> From<DigitalWrap> for Option<I>
where
  I: Integer + Unsigned + Bounded + FromPrimitive + Copy + Debug,
{
  fn from(base: DigitalWrap) -> Self {
    match base {
      DigitalWrap::Max => Some(I::max_value()),
      DigitalWrap::Ten => I::from_u8(9),
      DigitalWrap::U128(wrap) => I::from_u128(wrap - 1),
      DigitalWrap::U64(wrap) => I::from_u64(wrap - 1),
      DigitalWrap::U32(wrap) => I::from_u32(wrap - 1),
      DigitalWrap::U16(wrap) => I::from_u16(wrap - 1),
      DigitalWrap::U8(wrap) => I::from_u8(wrap - 1),
    }
  }
}
