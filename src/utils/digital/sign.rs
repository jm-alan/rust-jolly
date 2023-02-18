use std::cmp::Ordering;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Sign {
  #[default]
  Zero,
  Positive,
  Negative,
}

impl Sign {
  #[inline(always)]
  pub fn negated(&self) -> Self {
    match self {
      Self::Zero => Self::Zero,
      Self::Positive => Self::Negative,
      Self::Negative => Self::Positive,
    }
  }

  #[inline(always)]
  pub fn is_positive(self) -> bool {
    self == Self::Positive
  }

  #[inline(always)]
  pub fn is_negative(self) -> bool {
    self == Self::Negative
  }

  #[inline(always)]
  pub fn is_zero(self) -> bool {
    self == Self::Zero
  }
}

impl PartialOrd for Sign {
  #[inline(always)]
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    match (self, other) {
      (Self::Positive, Self::Zero) => Some(Ordering::Greater),
      (Self::Positive, Self::Negative) => Some(Ordering::Greater),
      (Self::Zero, Self::Negative) => Some(Ordering::Greater),
      (Self::Zero, Self::Positive) => Some(Ordering::Less),
      (Self::Negative, Self::Zero) => Some(Ordering::Less),
      (Self::Negative, Self::Positive) => Some(Ordering::Greater),
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
