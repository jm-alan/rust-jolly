use num::{Bounded, FromPrimitive, Integer, Unsigned};
use std::fmt::Debug;

use super::wrap::DigitalWrap;

mod fixed_width;
mod variable_width;

pub use fixed_width::*;
pub use variable_width::*;

#[inline(always)]
pub fn wrapping_add<I>(lhs: I, rhs: I, base: DigitalWrap) -> I
where
  I: Integer + Unsigned + Bounded + FromPrimitive + Copy + Debug,
{
  let option_wrap: Option<I> = base.into();
  let Some(wrap_point) = option_wrap else {
    panic!("Failed to create digital wrap point from supplied base");
  };

  let ceil = wrap_point - lhs;
  if rhs > ceil {
    rhs - ceil - I::one()
  } else {
    lhs + rhs
  }
}
