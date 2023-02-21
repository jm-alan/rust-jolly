mod fixed_width;
mod variable_width;

pub use fixed_width::*;
pub use variable_width::*;

#[inline(always)]
fn higher_order_multiply(lhs: u32, rhs: u32) -> u64 {
  lhs as u64 * rhs as u64
}

#[inline(always)]
fn fit_shift(val: u64, target: &mut u32) -> u32 {
  let ceil_val = u32::MAX as u64;
  if val <= ceil_val {
    *target = val as u32;
    0
  } else {
    *target = val as u32;
    (val >> 32) as u32
  }
}
