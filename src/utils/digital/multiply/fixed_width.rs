use super::{fit_shift, higher_order_multiply};
use crate::utils::{
  digital::add::digital_add_in_place_fixed_width, wrapping_add, DigitalWrap,
};

#[inline(always)]
pub fn digital_multiply_u32_into_fixed_width(
  lhs: &[u32],
  rhs: &[u32],
) -> [u32; 65536] {
  if lhs.len() + rhs.len() > u16::MAX as usize + 1 {
    panic!("Multiplication result too large");
  }

  let mut result = [0; 65536];

  match (lhs.len(), rhs.len()) {
    (0, _) => {
      result.copy_from_slice(lhs);
      result
    }
    (_, 0) => {
      result.copy_from_slice(rhs);
      result
    }
    (1, 1) => {
      let high_res = higher_order_multiply(lhs[0], rhs[0]);
      result[0] = high_res as u32;
      result[1] = (high_res >> 32) as u32;
      result
    }
    (1, _) => digital_scalar_multiply_out_of_place_u32_fixed_width(rhs, lhs[0]),
    (_, 1) => digital_scalar_multiply_out_of_place_u32_fixed_width(lhs, rhs[0]),
    (left_magnitude, right_magnitude) => {
      let result_magnitude = left_magnitude + right_magnitude;

      let standing_zeroes = vec![0; result_magnitude];
      let mut scaled_mult = [0; 65536];

      for (idx, digit) in rhs.iter().enumerate() {
        scaled_mult.copy_from_slice(&standing_zeroes);

        scaled_mult[idx..(idx + left_magnitude)]
          .copy_from_slice(&lhs[0..left_magnitude]);

        digital_scalar_multiply_in_place_u32_fixed_width(
          &mut scaled_mult,
          *digit,
        );

        digital_add_in_place_fixed_width(
          &mut result,
          &scaled_mult,
          DigitalWrap::Max,
        );
      }

      result
    }
  }
}

#[inline(always)]
pub fn digital_scalar_multiply_out_of_place_u32_fixed_width(
  lhs: &[u32],
  rhs: u32,
) -> [u32; 65536] {
  let mut result = [0; 65536];
  result.copy_from_slice(lhs);
  digital_scalar_multiply_in_place_u32_fixed_width(&mut result, rhs);
  result
}

#[inline(always)]
pub fn digital_scalar_multiply_in_place_u32_fixed_width(
  lhs: &mut [u32; 65536],
  rhs: u32,
) {
  let mut carry = 0;
  let mut zero_stop = u16::MAX as usize;

  while lhs[zero_stop] == 0 && zero_stop != 0 {
    zero_stop -= 1
  }

  if zero_stop == 0 {
    return;
  }

  for el in lhs.iter_mut().take(zero_stop) {
    let after_fit = fit_shift(higher_order_multiply(*el, rhs), el);

    let before_carry = *el;

    *el = wrapping_add(*el, carry, DigitalWrap::Max);

    carry = after_fit + (*el < before_carry) as u32;
  }
}
