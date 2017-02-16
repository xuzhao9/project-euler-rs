// Author: Xu Zhao (i@xuzhao.net)
// Date: Feb 16, 2017

extern crate num;
use self::num::{BigUint, One};
use self::num::FromPrimitive;


fn bignum_power(a: BigUint, b: BigUint) -> BigUint {
  let mut r = a.clone();
  let mut x = &b.clone() + &One::one();
  loop {
    if x == One::one() {
      break;
    }
    r = r * &a;
    x = &x - &One::one();
  }
  r
}

fn bignum_sumdigits(a: BigUint) -> i32 {
  let s = a.to_str_radix(10);
  let mut sum = 0;
  for x in s.as_bytes() {
    sum = sum + (x - '0' as u8) as i32;
  }
  sum
}

pub fn solve() {
  let mut f0: BigUint = One::one();
  let mut max = 0;
  for a in 1..100 {
    for b in 1..100 {
      let aa = BigUint::from_i32(a).unwrap();
      let bb = BigUint::from_i32(b).unwrap();
      let r = bignum_power(aa, bb);
      let ss = bignum_sumdigits(r);
      if ss > max {
        max = ss;
      }
    }
  }
  println!("max={}", max);
}

