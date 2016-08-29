// Author: Xu Zhao (i@xuzhao.net)
// Date: Aug 29, 2016

//! problem 38

// 9X
//

// 9XX

// 9XXX
// 4, 5

fn stitch(d1: i32, d2: i32, d3: i32) -> i32 {
  let mut digits:Vec<i32> = Vec::new();
  digits.push(9);
  if d1 != 0 {
    digits.push(d1);
  }
  if d2 != 0 {
    digits.push(d2);
  }
  if d3 != 0 {
    digits.push(d3);
  }
  let mut r = 0;
  for i in 0..digits.len() {
    r = r * 10 + digits[i];
  }
  return r;
}

fn product_concat(d: i32) -> i32 {
  let mut i = 1;
  let mut f = 0;
  loop {
    let p = d * i;
    if log10(p) + 1 + log10(f) + 1 < 10 {
      f = f * (10i32.pow((p as f64).log(10f64) as u32 + 1)) + p;
      i += 1;
    } else {
      break;
    }
  }
  return f;
}

fn log10(i: i32) -> i32 {
  if i == 0 {
    return 0;
  }
  return (i as f64).log(10f64) as i32;
}

fn is_pandigital(num: i32) -> bool {
  if log10(num) != 8 {
    return false;
  }
  let mut find: [bool; 10] = [false; 10];
  for i in 0..9 {
    let digit = (num / 10i32.pow(i as u32)) % 10;
    if digit == 0 {
      return false;
    }
    if find[digit as usize] == true {
      return false;
    } else {
      find[digit as usize] = true;
    }
  }
  return true;
}

pub fn solve() {
  let mut max = 0;
  let mut max_n = 0;
  for d1 in 0..10 {
    for d2 in 0..10 {
      for d3 in 0..10 {
        let n = stitch(d1, d2, d3);
        let p = product_concat(n);
        // println!("{}, {}", n, p);
        if is_pandigital(p) {
          if p > max {
            max = p;
            max_n = n;
          }
        }
      }
    }
  }
  println!("{}, {}", max, max_n);
}

