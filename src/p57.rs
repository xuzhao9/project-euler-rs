// Author: Xu Zhao (i@xuzhao.net)
// Date: Feb 16, 2017

extern crate num;
use self::num::{BigUint, One};
use self::num::FromPrimitive;

fn cal(n: BigUint, d: BigUint) -> (BigUint, BigUint) {
  let mut s: BigUint = One::one();
  let mut nn = n + d.clone();
  s = nn;
  nn = d.clone();
  let dd = s;
  nn = nn + dd.clone();
  (nn, dd)
}

pub fn solve() {
  let mut numerator = BigUint::from_i32(3).unwrap();
  let mut denominator = BigUint::from_i32(2).unwrap();
  let mut cumu = 0;
  for _ in 2..1001 {
    let (nn, dd) = cal(numerator, denominator);
    numerator = nn;
    denominator = dd;
    if numerator.to_str_radix(10).len() > denominator.to_str_radix(10).len() {
      cumu += 1;
    }
  }
  println!("cumu = {}", cumu);
}

