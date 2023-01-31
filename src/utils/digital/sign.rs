use std::cmp::Ordering;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Sign {
  #[default]
  Zero,
  Positive,
  Negative,
}

impl Sign {
  pub fn negate(&self) -> Self {
    use Sign::{Negative, Positive, Zero};
    match self {
      Zero => Zero,
      Positive => Negative,
      Negative => Positive,
    }
  }
}

impl PartialOrd for Sign {
  #[inline(always)]
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    use Sign::{Negative, Positive, Zero};
    match (self, other) {
      (Positive, Zero) => Some(Ordering::Greater),
      (Positive, Negative) => Some(Ordering::Greater),
      (Zero, Negative) => Some(Ordering::Greater),
      (Zero, Positive) => Some(Ordering::Less),
      (Negative, Zero) => Some(Ordering::Less),
      (Negative, Positive) => Some(Ordering::Greater),
      _ => Some(Ordering::Equal),
    }
  }
}

impl Ord for Sign {
  #[inline(always)]
  fn cmp(&self, other: &Self) -> Ordering {
    self.partial_cmp(other).unwrap()
  }
}
