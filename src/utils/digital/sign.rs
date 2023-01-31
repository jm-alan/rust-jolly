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
