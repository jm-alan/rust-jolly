pub trait FromBool {
  fn from_bool(init: bool) -> Self;
}

impl FromBool for i8 {
  #[inline(always)]
  fn from_bool(init: bool) -> Self {
    init as Self
  }
}

impl FromBool for u8 {
  #[inline(always)]
  fn from_bool(init: bool) -> Self {
    init as Self
  }
}

impl FromBool for i16 {
  #[inline(always)]
  fn from_bool(init: bool) -> Self {
    init as Self
  }
}

impl FromBool for u16 {
  #[inline(always)]
  fn from_bool(init: bool) -> Self {
    init as Self
  }
}

impl FromBool for i32 {
  #[inline(always)]
  fn from_bool(init: bool) -> Self {
    init as Self
  }
}

impl FromBool for u32 {
  #[inline(always)]
  fn from_bool(init: bool) -> Self {
    init as Self
  }
}

impl FromBool for i64 {
  #[inline(always)]
  fn from_bool(init: bool) -> Self {
    init as Self
  }
}

impl FromBool for u64 {
  #[inline(always)]
  fn from_bool(init: bool) -> Self {
    init as Self
  }
}

impl FromBool for i128 {
  #[inline(always)]
  fn from_bool(init: bool) -> Self {
    init as Self
  }
}

impl FromBool for u128 {
  #[inline(always)]
  fn from_bool(init: bool) -> Self {
    init as Self
  }
}
