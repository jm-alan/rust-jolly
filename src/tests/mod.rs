use crate::{
  bigint::BigInt,
  utils::{digital_add, digital_subtract, DigitalWrap},
};

// #[test]
// fn test_addition() {
//   let mut num = BigInt::zero();
//   num += u64::MAX;
//   println!("{num}");
//   num += 1;
//   println!("{num}");
// }

#[test]
fn test_digital_add() {
  let left_u8_base10 = vec![0, 0, 0, 1u8];
  let right_u8_base10 = vec![9, 9, 9, 9];
  let result_u8_base10 =
    digital_add(&left_u8_base10, &right_u8_base10, DigitalWrap::Ten);
  assert_eq!(result_u8_base10, vec![9, 9, 9, 0, 1]);

  let left_u8_base16 = vec![9, 13, 10, 15u8];
  let right_u8_base16 = vec![15, 10, 13, 7];
  let result_u8_base16 =
    digital_add(&left_u8_base16, &right_u8_base16, DigitalWrap::U8(16));
  assert_eq!(result_u8_base16, vec![8, 8, 8, 7, 1]);

  let left_u16_base65536 = vec![54321, 12345, 54321, 12345u16];
  let right_u16_base65536 = vec![54321, 12345, 54321, 12345];
  let result_u16_base65536 =
    digital_add(&left_u16_base65536, &right_u16_base65536, DigitalWrap::Max);
  assert_eq!(result_u16_base65536, vec![43106, 24691, 43106, 24691]);
}

// #[test]
// fn test_digital_subtract() {
//   let left_u8_base10 = vec![0, 0, 0, 1u8];
//   let right_u8_base10 = vec![9, 9, 9, 9];
//   let result_u8_base10 =
//     digital_subtract(&left_u8_base10, &right_u8_base10, DigitalWrap::Ten);
//   assert_eq!(result_u8_base10, vec![9, 9, 9, 8]);

//   let left_u8_base16 = vec![9, 13, 10, 15u8];
//   let right_u8_base16 = vec![15, 10, 13, 7];
//   let result_u8_base16 =
//     digital_subtract(&left_u8_base16, &right_u8_base16, DigitalWrap::U8(16));
//   assert_eq!(result_u8_base16, vec![10, 2, 13, 7]);

//   let left_u16_base65536 = vec![54321, 12345, 54321, 12345u16];
//   let right_u16_base65536 = vec![54321, 12345, 54321, 12345];
//   let result_u16_base65536 = digital_subtract(
//     &left_u16_base65536,
//     &right_u16_base65536,
//     DigitalWrap::Max,
//   );
//   assert_eq!(result_u16_base65536, vec![0]);
// }
