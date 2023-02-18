mod digital;

pub use digital::{
  digital_add, digital_add_in_place, digital_cmp,
  digital_scalar_divide_in_place_u32, digital_scalar_divide_u32,
  digital_scalar_multiply_in_place_u32, digital_scalar_rem_u32,
  digital_subtract, Digital, DigitalWrap, Sign,
};
