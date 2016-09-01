// Author: Xu Zhao (i@xuzhao.net)
// Date: Sep  1, 2016

//! problem 47

extern crate primes;

pub fn solve() {
  let mut x = 644;
  loop {
    let xs: [u64; 4] = [x, x+1, x+2, x+3];
    let factors: Vec<Vec<u64>> = xs.iter().map(|x| primes::factors_uniq(*x)).collect();
    let mut flag = true;
    for fac in factors.iter() {
      if fac.len() != 4 {
        flag = false;
      }
    }
    if flag {
      println!("{}", x);
      break;
    }
    x += 1;
  }
}
