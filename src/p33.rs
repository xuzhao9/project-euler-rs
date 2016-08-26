// Author: Xu Zhao (i@xuzhao.net)
// Date: Aug 26, 2016

//! problem 33

fn find_common(n1: i32, n2: i32) -> Option<i32> {
  let digit1 = n1 / 10;
  let digit1_2 = n1 % 10;
  let digit2 = n2 / 10;
  let digit2_2 = n2 % 10;
  if digit1 == digit2 {
    Some(digit1)
  } else if digit1_2 == digit2 {
    Some(digit1_2)
  } else if digit1 == digit2_2 {
    Some(digit1)
  } else if digit1_2 == digit2_2 {
    Some(digit1_2)
  } else {
    None
  }
}

fn remove_num(n1: i32, n2: i32) -> i32 {
  let digit1 = n1 / 10;
  let digit1_2 = n1 % 10;
  if n2 == digit1 {
    return digit1_2;
  } else if n2 == digit1_2 {
    return digit1;
  } else {
    panic!("Cannot find {} in {}", n2, n1);
  }
}

fn find_greatest_common_divisor(n1: i32, n2: i32) -> i32 {
  // make sure n1 > n2
  if n2 == 0 {
    return n1;
  } else {
    return find_greatest_common_divisor(n2, n1 % n2);
  }
}

pub fn solve() {
  let mut product_numerator = 1;
  let mut product_demoninator = 1;
  for d1 in 10..100 {
    for d2 in 10..d1 {
      match find_common(d1, d2) {
        Some(c) => {
          if c != 0 {
            let n3 = remove_num(d1, c);
            let n4 = remove_num(d2, c);
            if n4 * d1 == d2 * n3 {
              product_numerator *= d2;
              product_demoninator *= d1;
              println!("{}/{}", d2, d1);
            }
          }
        },
        None => {
          // do nothing
        },
      }
    }
  }
  // find greatest common divisor
  println!("{}",
           product_demoninator / find_greatest_common_divisor(product_demoninator, product_numerator));
}

