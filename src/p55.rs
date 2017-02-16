// Author: Xu Zhao (i@xuzhao.net)
// Date: Feb 15, 2017

extern crate num;
use self::num::{BigUint, One};

fn is_lychrel(x: &BigUint) -> bool {
  let mut xx = x.clone();
  for _ in 0..50 {
    let s = xx.to_str_radix(10);
    let srev = s.chars().rev().collect::<String>();
    let y = BigUint::parse_bytes(srev.as_bytes(), 10).unwrap();
    let n = &xx + y;
    let sn = n.to_str_radix(10);
    let sn_rev = sn.chars().rev().collect::<String>();
    if sn_rev == sn {
      return false; // not a lychrel number
    }
    xx = n.clone();
  }
  true
}

pub fn solve() {
  let mut f0: BigUint = One::one();
  let mut cnt = 0;
  for _ in 0..10000 {
    if is_lychrel(&f0) {
      println!("{} is lychrel!", f0);
      cnt += 1;
    }
    f0 = &f0 + &One::one();
  }
  println!("{}", cnt);
}

