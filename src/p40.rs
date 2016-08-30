// Author: Xu Zhao (i@xuzhao.net)
// Date: Aug 29, 2016

//! problem 40

fn num_to_digits(num: u64) -> Vec<u64> {
  let mut nn = num.clone();
  let mut v: Vec<u64> = Vec::new();
  loop {
    if nn == 0 {
      v.reverse();
      return v;
    }
    let d = nn % 10;
    v.push(d);
    nn = nn / 10;
  }
}

fn contains(arr: &[i32; 7], n: i32) -> bool {
  for i in arr {
    if n == *i {
      return true;
    }
  }
  return false;
}

pub fn solve() {
  let mut p1 = 0;
  let mut num = 1;
  let arr: [i32; 7] = [1, 10, 100, 1000, 10000, 100000, 1000000];
  let mut prod = 1;
  'outer: loop {
    let v = num_to_digits(num);
    'inner: for i in 0..v.len() {
      p1 += 1;
      if contains(&arr, p1) {
        prod *= v[i];
      }
      if p1 == arr[6] {
        println!("{}", prod);
        break 'outer;
      }
    }
    num += 1;
  }
}
