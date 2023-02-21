use std::ops::{Mul, MulAssign};

use crate::{
  bigint::BigInt,
  utils::{digital_multiply_u32, karatsuba_mul},
};

impl Mul for &BigInt {
  type Output = BigInt;

  #[inline(always)]
  fn mul(self, rhs: &BigInt) -> Self::Output {
    let mut result = self.clone();

    result *= rhs;

    result
  }
}

impl MulAssign<&BigInt> for BigInt {
  #[inline(always)]
  fn mul_assign(&mut self, rhs: &BigInt) {
    if rhs.is_zero() {
      return self.zero_out();
    }

    self.digits = if self.magnitude() > Self::karatsuba_crossover()
      && rhs.magnitude() > Self::karatsuba_crossover()
    {
      karatsuba_mul(&self.digits, &rhs.digits, Self::karatsuba_crossover())
    } else {
      digital_multiply_u32(&self.digits, &rhs.digits)
    };

    if rhs.sign.is_negative() {
      self.sign = self.sign.negated();
    }
  }
}

impl BigInt {
  #[inline(always)]
  pub fn pow(&self, rhs: u64) -> Self {
    let mut result = self.clone();

    result.pow_assign(rhs);

    result
  }

  #[inline(always)]
  pub fn pow_assign(&mut self, rhs: u64) {
    for _ in 1..rhs {
      *self *= &rhs.into();
    }
  }
}
