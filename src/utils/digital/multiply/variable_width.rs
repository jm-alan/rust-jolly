use std::ops::Mul;

use crate::utils::{
  digital_add, digital_add_in_place, digital_subtract, wrapping_add,
  DigitalWrap, Sign,
};

use super::{fit_shift, higher_order_multiply};

#[inline(always)]
pub fn digital_multiply_u32(lhs: &[u32], rhs: &[u32]) -> Vec<u32> {
  match (lhs.len(), rhs.len()) {
    (0, _) => rhs.to_owned(),
    (_, 0) => lhs.to_owned(),
    (1, 1) => {
      let high_res = higher_order_multiply(lhs[0], rhs[0]);
      if high_res > u32::MAX as u64 {
        vec![high_res as u32, (high_res >> 32) as u32]
      } else {
        vec![high_res as u32]
      }
    }
    (1, _) => digital_scalar_multiply_u32(rhs, lhs[0]),
    (_, 1) => digital_scalar_multiply_u32(lhs, rhs[0]),
    (left_magnitude, right_magnitude) => {
      let result_magnitude = left_magnitude + right_magnitude;

      let standing_zeroes = vec![0; result_magnitude];
      let mut scaled_mult = vec![0; result_magnitude];
      let mut result = vec![0; result_magnitude];

      for (idx, digit) in rhs.iter().enumerate() {
        scaled_mult.copy_from_slice(&standing_zeroes);

        scaled_mult[idx..(idx + left_magnitude)]
          .copy_from_slice(&lhs[0..left_magnitude]);

        digital_scalar_multiply_in_place_u32(&mut scaled_mult, *digit);

        digital_add_in_place(&mut result, &scaled_mult, DigitalWrap::Max);
      }

      result
    }
  }
}

#[inline(always)]
pub fn digital_scalar_multiply_in_place_u32(lhs: &mut Vec<u32>, rhs: u32) {
  let mut carry = 0;

  for el in lhs.iter_mut() {
    let after_fit = fit_shift(higher_order_multiply(*el, rhs), el);

    let before_carry = *el;

    *el = wrapping_add(*el, carry, DigitalWrap::Max);

    carry = after_fit + (*el < before_carry) as u32;
  }

  if carry > 0 {
    lhs.push(carry);
  }
}

#[inline(always)]
pub fn digital_scalar_multiply_u32(lhs: &[u32], rhs: u32) -> Vec<u32> {
  let mut clone = lhs.to_vec();
  digital_scalar_multiply_in_place_u32(&mut clone, rhs);
  clone
}

impl Mul for Sign {
  type Output = Self;

  fn mul(self, rhs: Self) -> Self::Output {
    match rhs {
      Sign::Negative => self.negated(),
      Sign::Zero => Sign::Zero,
      _ => self,
    }
  }
}

#[inline(always)]
pub fn karatsuba_mul(lhs: &[u32], rhs: &[u32], crossover: usize) -> Vec<u32> {
  match (lhs.len(), rhs.len()) {
    (0, _) | (_, 0) | (1, _) | (_, 1) => digital_multiply_u32(lhs, rhs),
    (left_magnitude, right_magnitude) => {
      let should_k_recursive =
        left_magnitude >= crossover && right_magnitude >= crossover;

      let result_magnitude = left_magnitude + right_magnitude;
      let mut result = Vec::with_capacity(result_magnitude);

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

      let mut digital_ac = if should_k_recursive {
        karatsuba_mul(&a, &c, crossover)
      } else {
        digital_multiply_u32(&a, &c)
      };

      let digital_bd = if should_k_recursive {
        karatsuba_mul(&b, &d, crossover)
      } else {
        digital_multiply_u32(&b, &d)
      };

      let mut ad_bc_final = digital_subtract(
        &digital_subtract(
          &if should_k_recursive {
            karatsuba_mul(
              &digital_add(&a, &b, DigitalWrap::Max),
              &digital_add(&c, &d, DigitalWrap::Max),
              crossover,
            )
          } else {
            digital_multiply_u32(
              &digital_add(&a, &b, DigitalWrap::Max),
              &digital_add(&c, &d, DigitalWrap::Max),
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

      let mut half_zeroes = vec![0; half];
      let mut full_zeroes = vec![0; half * 2];

      half_zeroes.append(&mut ad_bc_final);
      full_zeroes.append(&mut digital_ac);

      result.extend(digital_bd);

      digital_add_in_place(&mut result, &half_zeroes, DigitalWrap::Max);
      digital_add_in_place(&mut result, &full_zeroes, DigitalWrap::Max);

      result
    }
  }
}
