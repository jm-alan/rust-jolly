use std::cmp::Ordering;

#[inline(always)]
pub fn digital_scalar_divide_in_place_u32(lhs: &mut [u32], rhs: u32) -> u32 {
  if rhs == 0 {
    panic!("Attempt to divide by zero");
  } else if lhs.is_empty() {
    panic!("Attempt to divide nonexistent number");
  } else if lhs.len() == 1 {
    let dividend = lhs[0];
    let (quot, rem) = (dividend / rhs, dividend % rhs);
    lhs[0] = quot;
    rem
  } else {
    let mut digit_considering = lhs.len() - 1;
    let mut rem = 0;
    while digit_considering != usize::MAX {
      let (quot, current_rem) =
        digital_u64_u32_divide_with_rem(&[lhs[digit_considering], rem], rhs);
      rem = current_rem;
      match quot.cmp(&(u32::MAX as u64)) {
        Ordering::Greater => {
          lhs[digit_considering] = quot as u32;
          lhs[digit_considering + 1] = (quot >> 32) as u32;
        }
        _ => lhs[digit_considering] = quot as u32,
      }
      digit_considering = digit_considering.wrapping_sub(1);
    }
    rem
  }
}

#[inline(always)]
fn digital_u64_u32_divide_with_rem(lhs: &[u32], rhs: u32) -> (u64, u32) {
  let combined = match lhs.len() {
    1 => lhs[0] as u64,
    2 => lhs[0] as u64 + ((lhs[1] as u64) << 32),
    _ => panic!("Attempt to divide inordinatedly sized value as u64"),
  };
  let cast_divisor = rhs as u64;
  let (quot, rem) = (combined / cast_divisor, combined % cast_divisor);
  (quot, rem as u32)
}

#[inline(always)]
pub fn digital_scalar_divide_u32(lhs: &[u32], rhs: u32) -> (Vec<u32>, u32) {
  let mut cloned = lhs.to_vec();
  let rem = digital_scalar_divide_in_place_u32(&mut cloned, rhs);
  (cloned, rem)
}

#[inline(always)]
pub fn digital_scalar_rem_u32(lhs: &[u32], rhs: u32) -> u32 {
  digital_scalar_divide_in_place_u32(&mut lhs.to_vec(), rhs)
}
