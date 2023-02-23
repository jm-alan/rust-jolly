use super::{fit_shift, higher_order_multiply};
use crate::utils::{
  digital::add::digital_add_in_place_fixed_width, digital_add_fixed_width,
  digital_subtract_fixed_width, wrapping_add, DigitalWrap,
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
    (0, _) => result.copy_from_slice(lhs),
    (_, 0) => result.copy_from_slice(rhs),
    (1, 1) => {
      let high_res = higher_order_multiply(lhs[0], rhs[0]);
      result[0] = high_res as u32;
      result[1] = (high_res >> 32) as u32;
    }
    (1, _) => {
      result.copy_from_slice(rhs);
      digital_scalar_multiply_in_place_u32_fixed_width(&mut result, lhs[0]);
    }
    (_, 1) => {
      result.copy_from_slice(lhs);
      digital_scalar_multiply_in_place_u32_fixed_width(&mut result, rhs[0]);
    }
    (left_magnitude, _) => {
      let mut scaled_mult = [0; 65536];

      for (idx, digit) in rhs.iter().enumerate() {
        scaled_mult[0..65536].fill(0);

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
    }
  };
  result
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

#[inline(always)]
pub fn karatsuba_mul_fixed_width(
  lhs: &[u32],
  rhs: &[u32],
  crossover: usize,
) -> [u32; 65536] {
  match (lhs.len(), rhs.len()) {
    (0, _) | (_, 0) | (1, _) | (_, 1) => {
      digital_multiply_u32_into_fixed_width(lhs, rhs)
    }
    (left_magnitude, right_magnitude) => {
      let should_k_recursive =
        left_magnitude >= crossover && right_magnitude >= crossover;

      let mut result = [0; 65536];

      let larger_magnitude = usize::max(left_magnitude, right_magnitude);
      let half = larger_magnitude >> 1;

      let a: Vec<u32> = lhs
        .iter()
        .copied()
        .skip(half)
        .take(lhs.len().wrapping_sub(half))
        .collect();
      let b: Vec<u32> = lhs.iter().copied().take(half).collect();
      let c: Vec<u32> = rhs
        .iter()
        .copied()
        .skip(half)
        .take(rhs.len().wrapping_sub(half))
        .collect();
      let d: Vec<u32> = rhs.iter().copied().take(half).collect();

      let digital_ac = if should_k_recursive {
        karatsuba_mul_fixed_width(&a, &c, crossover)
      } else {
        digital_multiply_u32_into_fixed_width(&a, &c)
      };

      let digital_bd = if should_k_recursive {
        karatsuba_mul_fixed_width(&b, &d, crossover)
      } else {
        digital_multiply_u32_into_fixed_width(&b, &d)
      };

      let ad_bc_final = digital_subtract_fixed_width(
        &digital_subtract_fixed_width(
          &if should_k_recursive {
            karatsuba_mul_fixed_width(
              &digital_add_fixed_width(&a, &b, DigitalWrap::Max),
              &digital_add_fixed_width(&c, &d, DigitalWrap::Max),
              crossover,
            )
          } else {
            digital_multiply_u32_into_fixed_width(
              &digital_add_fixed_width(&a, &b, DigitalWrap::Max),
              &digital_add_fixed_width(&c, &d, DigitalWrap::Max),
            )
          },
          &digital_ac,
          DigitalWrap::Max,
        )
        .0,
        &digital_bd,
        DigitalWrap::Max,
      )
      .0;

      let mut half_zeroes = [0; 65536];
      let mut full_zeroes = [0; 65536];

      let mut adbc_zero_stop = 65535;
      while ad_bc_final[adbc_zero_stop] == 0 && adbc_zero_stop > 0 {
        adbc_zero_stop -= 1;
      }

      let mut ac_zero_stop = 65535;
      while digital_ac[ac_zero_stop] == 0 && ac_zero_stop > 0 {
        ac_zero_stop -= 1;
      }

      half_zeroes[half..65536]
        .copy_from_slice(&ad_bc_final[0..=adbc_zero_stop]);

      full_zeroes[(half * 2)..65536]
        .copy_from_slice(&digital_ac[0..=ac_zero_stop]);

      result.copy_from_slice(&digital_bd);

      digital_add_in_place_fixed_width(
        &mut result,
        &half_zeroes,
        DigitalWrap::Max,
      );
      digital_add_in_place_fixed_width(
        &mut result,
        &full_zeroes,
        DigitalWrap::Max,
      );

      result
    }
  }
}
