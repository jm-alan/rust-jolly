mod add;
mod cmp;
mod divide;
mod multiply;
mod sign;
mod subtract;
mod wrap;

pub use self::{
  add::{digital_add, digital_add_in_place},
  cmp::digital_cmp,
  divide::{digital_scalar_divide_in_place_u32, digital_scalar_divide_u32},
  multiply::digital_scalar_multiply_in_place_u32,
  sign::Sign,
  subtract::digital_subtract,
  wrap::DigitalWrap,
};

pub trait Digital {
  fn as_digits(&self) -> Vec<u8>;
}

impl Digital for u64 {
  #[inline(always)]
  fn as_digits(&self) -> Vec<u8> {
    if self < &10 {
      return vec![*self as u8];
    }
    let mut copied = *self;

    let mut result = vec![];

    while copied > 0 {
      result.push((copied % 10) as u8);
      copied /= 10
    }

    result
  }
}

impl Digital for u32 {
  #[inline(always)]
  fn as_digits(&self) -> Vec<u8> {
    if self < &10 {
      return vec![*self as u8];
    }
    let mut copied = *self;

    let mut result = vec![];

    while copied > 0 {
      result.push((copied % 10) as u8);
      copied /= 10
    }

    result
  }
}
