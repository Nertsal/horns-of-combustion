mod real;

use geng::prelude::R32;

pub use self::real::*;

fn approach(a: R32, b: R32, shift: R32) -> R32 {
  if a < b {
    return b.min(a + shift);
  }
  return b.max(a - shift);
}