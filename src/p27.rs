// Author: Xu Zhao (i@xuzhao.net)
// Date: Aug 20, 2016

//! problem 27

fn is_prime(n: i32) -> bool {
  let upper = (n as f32).sqrt() as i32 + 1;
  for i in 2..upper+1 {
    if n % i == 0 {
      return false;
    }
  }
  return true;
}

fn get_t(a: i32, b: i32) -> i32 {
  let mut n = 0;
  loop {
    let f = n * n + a * n + b;
    if f > 0 && is_prime(f) {
      n += 1;
    } else {
      return n-1;
    }
  }
}


pub fn solve() {
  let mut max = 0;
  let mut a: i32 = 0;
  let mut b: i32 = 0;
  for i in -1000..1000 {
    for j in 0..1001 {
      if max < get_t(i, j) && i.abs() % 2 == 1 {
        max = get_t(i, j);
        a = i;
        b = j;
      }
    }
  }
  println!("{}, {}, {} -> {}", a, b, max, a * b);
}



