mod add;
mod cmp;
mod sign;
mod subtract;
mod wrap;

pub use self::{
  add::digital_add, cmp::digital_cmp, sign::Sign, subtract::digital_subtract,
  wrap::DigitalWrap,
};

pub trait Digital {
  fn as_digits(&self) -> Vec<u8>;
}
