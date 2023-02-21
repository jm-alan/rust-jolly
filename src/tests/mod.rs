use std::time::Instant;

use crate::{
  bigint::BigInt,
  utils::{
    digital_add, digital_add_in_place, digital_multiply_u32,
    digital_scalar_divide_in_place_u32, digital_subtract, karatsuba_mul,
    DigitalWrap, Sign,
  },
};

#[test]
fn test_display() {
  // let mut num = BigInt::zero();
  let target_fact = 100000u64;
  let hundo = BigInt::from(target_fact);

  let timer = Instant::now();
  let hundo_facto = hundo.fact();
  let finished = timer.elapsed();

  println!("Computed {target_fact}! in {:?}", finished);
  println!(
    "{target_fact}! requires a BigInt magnitude of {}",
    hundo_facto.magnitude()
  );

  // for _ in 0..65536 {
  //   num += u32::MAX;
  // }

  // println!("\n4294967295 * 65536 = {num}\n");
  // println!("100! = {hundo_facto}\n");
}

#[test]
fn something() {
  let num = u64::MAX << 1;
  println!("{num}");
}

#[test]
fn test_digital_add() {
  let left_u8_base10 = vec![0, 0, 0, 1u8];
  let right_u8_base10 = vec![9, 9, 9, 9];

  let left_u8_base16 = vec![9, 13, 10, 15u8];
  let right_u8_base16 = vec![15, 10, 13, 7];

  let left_u16_base65536 = vec![54321, 12345, 54321, 12345u16];
  let right_u16_base65536 = vec![54321, 12345, 54321, 12345];

  let left_u64_baseu64 = vec![u64::MAX];
  let right_u64_baseu64 = vec![1];

  let big_left_u8_base10 = BigInt::u32_max_digits();
  let big_right_u8_base10 = BigInt::u32_max_digits();

  let result_u8_base10 =
    digital_add(&left_u8_base10, &right_u8_base10, DigitalWrap::Ten);
  let result_u8_base16 =
    digital_add(&left_u8_base16, &right_u8_base16, DigitalWrap::U8(16));
  let result_u16_base65536 =
    digital_add(&left_u16_base65536, &right_u16_base65536, DigitalWrap::Max);
  let result_u64_baseu64 =
    digital_add(&left_u64_baseu64, &right_u64_baseu64, DigitalWrap::Max);
  let big_result_u8_base10 =
    digital_add(&big_left_u8_base10, &big_right_u8_base10, DigitalWrap::Ten);

  assert_eq!(result_u8_base10, vec![9, 9, 9, 0, 1]);
  assert_eq!(result_u8_base16, vec![8, 8, 8, 7, 1]);
  assert_eq!(result_u16_base65536, vec![43106, 24691, 43106, 24691]);
  assert_eq!(result_u64_baseu64, vec![0, 1]);
  assert_eq!(big_result_u8_base10, vec![0, 9, 5, 4, 3, 9, 9, 8, 5, 8]);
}

#[test]
fn test_add_in_place() {
  let mut left_u8_base10 = vec![0, 0, 0, 1u8];
  let right_u8_base10 = vec![9, 9, 9, 9];

  let mut left_u8_base16 = vec![9, 13, 10, 15u8];
  let right_u8_base16 = vec![15, 10, 13, 7];

  let mut left_u16_base65536 = vec![54321, 12345, 54321, 12345u16];
  let right_u16_base65536 = vec![54321, 12345, 54321, 12345];

  let mut big_left_u8_base10 = BigInt::u32_max_digits().to_vec();
  let big_right_u8_base10 = BigInt::u32_max_digits();

  digital_add_in_place(&mut left_u8_base10, &right_u8_base10, DigitalWrap::Ten);
  digital_add_in_place(
    &mut left_u8_base16,
    &right_u8_base16,
    DigitalWrap::U8(16),
  );
  digital_add_in_place(
    &mut left_u16_base65536,
    &right_u16_base65536,
    DigitalWrap::Max,
  );
  digital_add_in_place(
    &mut big_left_u8_base10,
    &big_right_u8_base10,
    DigitalWrap::Ten,
  );

  assert_eq!(left_u8_base10, vec![9, 9, 9, 0, 1]);
  assert_eq!(left_u8_base16, vec![8, 8, 8, 7, 1]);
  assert_eq!(left_u16_base65536, vec![43106, 24691, 43106, 24691]);
  assert_eq!(big_left_u8_base10, vec![0, 9, 5, 4, 3, 9, 9, 8, 5, 8]);
}

#[test]
fn test_digital_subtract() {
  let left_u8_base10 = vec![0, 0, 0, 1u8];
  let right_u8_base10 = vec![9, 9, 9, 9];

  let left_u8_multiple_borrow = vec![2, 1, 0, 8u8];
  let right_u8_multiple_borrow = vec![9, 9, 9, 1];

  let left_u8_base16 = vec![9, 13, 10, 15u8];
  let right_u8_base16 = vec![15, 10, 13, 7];

  let left_u16_base65536 = vec![54321, 12345, 54321, 12345u16];
  let right_u16_base65536 = vec![54321, 12345, 54321, 12345];

  let (result_u8_base10, sign_u8_base10) =
    digital_subtract(&left_u8_base10, &right_u8_base10, DigitalWrap::Ten);

  let (result_u8_multiple_borrow, sign_u8_multiple_borrow) = digital_subtract(
    &left_u8_multiple_borrow,
    &right_u8_multiple_borrow,
    DigitalWrap::Ten,
  );

  let (result_u8_base16, sign_u8_base16) =
    digital_subtract(&left_u8_base16, &right_u8_base16, DigitalWrap::U8(16));

  let (result_u16_base65536, sign_u16_base65536) = digital_subtract(
    &left_u16_base65536,
    &right_u16_base65536,
    DigitalWrap::Max,
  );

  assert_eq!(result_u8_base10, vec![9, 9, 9, 8]);
  assert_eq!(sign_u8_base10, Sign::Negative);

  assert_eq!(result_u8_multiple_borrow, vec![3, 1, 0, 6]);
  assert_eq!(sign_u8_multiple_borrow, Sign::Positive);

  assert_eq!(result_u8_base16, vec![10, 2, 13, 7]);
  assert_eq!(sign_u8_base16, Sign::Positive);

  assert_eq!(result_u16_base65536, vec![0]);
  assert_eq!(sign_u16_base65536, Sign::Zero);
}

#[test]
fn test_bigint_scalar_multiply() {
  let mut some_val = BigInt::from(1000u64);
  some_val *= 3;
  assert_eq!(format!("{some_val}"), "3000");
}

#[test]
fn test_digital_scalar_multiply() {
  let mut some_bigint = BigInt::from(u32::MAX / 5);

  some_bigint *= 11u32;
  assert_eq!(format!("{some_bigint}"), "9448928049");
}

#[test]
fn test_bigint_bigint_multiply() {
  let num_one = BigInt {
    sign: Sign::Positive,
    digits: vec![65536, 65536],
  };

  let num_two = BigInt {
    sign: Sign::Positive,
    digits: vec![65536, 65536],
  };

  let num_three = &num_one * &num_two;

  println!("{:?}", num_three.digits);
}

#[test]
fn test_factorial() {
  let hundo: BigInt = 100u64.into();
  let hundo_fact = hundo.fact();
  println!("{:?}", hundo_fact.digits);
}

#[test]
fn test_division() {
  let mut some_digits = vec![5, 10, 15, 20, 25];
  let mut some_more_digits = vec![5, 10, 15, 20, 25];
  digital_scalar_divide_in_place_u32(&mut some_digits, 5);
  digital_scalar_divide_in_place_u32(&mut some_more_digits, 20);
  println!("{:?}", some_digits);
  println!("{:?}", some_more_digits);
}

#[test]
fn test_karatsuba_multiplication() {
  let digits = 100000;
  let digit = u32::MAX / 2;
  let crossover = 60;

  let test_vec = vec![digit; digits];

  let some_digits = test_vec.clone();
  let some_other_digits = test_vec.clone();

  let mut timer = Instant::now();
  let result = karatsuba_mul(&some_digits, &some_other_digits, crossover);
  let k_time = timer.elapsed();

  // timer = Instant::now();
  // let digital_result = digital_multiply_u32(&some_digits, &some_other_digits);
  // let reg_time = timer.elapsed();

  // assert_eq!(result, digital_result);
  // println!(
  //   "Karatsuba took {:?}\n\
  //   Tranditional mult took {:?}\n\
  //   with a crossover of {crossover} multiplying {digits} digits of {digit}",
  //   k_time, reg_time
  // );
}
