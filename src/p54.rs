// Author: Xu Zhao (i@xuzhao.net)
// Date: Feb  14, 2017

use std::cmp;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::path::Path;

// const FILENAME: &'static str = "data/p054_test.txt";
 const FILENAME: &'static str = "data/p054_poker.txt";

fn find(arr: &[&str], c: char) -> bool {
  for a in arr {
    if a.as_bytes()[0] == c as u8 {
      return true;
    }
  }
  return false;
}

fn get_times(arr: &[&str], i: usize) -> usize {
  let mut c = 0 as usize;
  for a in arr {
    if a.as_bytes()[0] == arr[i].as_bytes()[0] {
      c += 1;
    }
  }
  c
}

fn get_suit_times(arr: &[&str], i: usize) -> usize {
  let mut c = 0 as usize;
  for a in arr {
    if a.as_bytes()[1] == arr[i].as_bytes()[1] {
      c += 1;
    }
  }
  c
}

fn get_value_times(arr: &[&str], i: i32) -> usize {
  let mut c = 0 as usize;
  for a in arr {
    let v = get_value(a);
    if v == i {
      c += 1;
    }
  }
  c
}

// get kind
fn get_value(s: &str) -> i32 {
  let c = s.as_bytes()[0].clone() as char;
  if c == 'T' {
    return 10;
  } else if c == 'J' {
    return 11;
  } else if c == 'Q' {
    return 12;
  } else if c == 'K' {
    return 13;
  } else if c == 'A' {
    return 14;
  } else {
    return (c as u8 - '0' as u8) as i32;
  }
}

fn get_suit(s: &str) -> u8 {
  s.as_bytes()[1].clone()
}

// Diamond, Spade, Heart, Club
fn cal(arr: &[&str]) -> (i32, i32) {
  // if royal flush?
  let mut same_suit = true;
  for x in 1..arr.len() {
    let c = arr[0].as_bytes()[1];
    if arr[x].as_bytes()[1] != c {
      same_suit = false;
    }
  }
  
  if same_suit && find(arr, 'T') && find(arr, 'J') && find(arr, 'Q') && find(arr, 'K') && find(arr, 'A') {
    return (10, 0);
  }
  // if straight flush? no, manuall verified
  // if four of a kind?
  let max = (0..5).fold((0, 0), |acc, x| {if get_times(arr, x) > acc.0 {(get_times(arr, x), get_value(arr[x]))} else {acc}});
  // println!("max={:?}", max);
  if max.0 == 4 {
    return (8, max.1);
  }

  if max.0 == 3 {
    for i in 0..5 {
      let x = get_times(arr, i);
      if x == 2 {
        //  println!("Full house with three {} s!", max.1);
        return (7, max.1); // return three
      }
    }
  }

  // Flush
  let suit_max = (0..5).fold((0, 0), |acc, x| {if get_suit_times(arr, x) > acc.0 {(get_suit_times(arr, x), get_value(arr[x]))} else {acc}});
  let number_max = (0..5).fold(0, |m, x| if get_value(arr[x]) > m {get_value(arr[x])} else {m});
  if suit_max.0 == 5 {
    return (6, number_max);
  }

  // Straight
  let mut straight = true;
  for i in (number_max-4)..(number_max+1) {
    let time = get_value_times(arr, i);
    if time != 1 {
      straight = false;
    }
  }
  if straight {
    return (5, number_max);
  }

  // Three of a kind
  if max.0 == 3 {
    return (4, max.1)
  }

  // Two pairs
  let mut v = Vec::new();
  for i in 0..5 {
    let time = get_times(arr, i);
    if time == 2 && !v.contains(&get_value(arr[i])){      
      v.push(get_value(arr[i]));
    }
  }
  if v.len() == 2 {
    if v[0] > v[1] {
      return (3, v[0]);  
    } else {
      return (3, v[1]);
    }
  } else if v.len() == 1 {
    return (2, v[0])
  }

  // High card
  (1,number_max)
}

fn compare(left: &[&str], right: &[&str]) -> bool {
  let r1 = cal(left);
  let r2 = cal(right);
  if r1.0 > r2.0 {
    return true;
  } else if r1.0 < r2.0 {
    return false;
  } else if r1.1 > r2.1 {
    return true;
  } else if r1.1 == r2.1 {
    if r1.0 == 2 {
      println!("tie in pair! {:?} vs {:?}", left, right);
      let left_number_max = (0..5).fold(0, |m, x| if
      get_value(left[x]) > m {get_value(left[x])} else {m});
      let right_number_max = (0..5).fold(0, |m, x| if
      get_value(right[x]) > m {get_value(right[x])} else {m});
      return left_number_max > right_number_max; 

    } else {
      println!("tie in : {:?} vs {:?}, {}", left, right, r1.0);
    }
  }
  return false;
}

pub fn solve() {
  let file = File::open(FILENAME).unwrap();
  let reader = BufReader::new(&file);
  let mut cnt = 0;
  for line in reader.lines() {
    let l = line.unwrap();
    let arr = l.split_whitespace().collect::<Vec<&str>>();
    let (left, right) = arr.split_at(5);
    if compare(left, right) {
      cnt += 1;
    }
  }
  println!("{}", cnt);
}

