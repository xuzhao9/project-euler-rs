// Author: Xu Zhao (i@xuzhao.net)
// Date: Feb 16, 2017

use std::cmp;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Read;
use std::char;
use std::path::Path;


const FILENAME: &'static str = "data/p059_cipher.txt";
  

pub fn is_alphabet(x: i32) -> bool {
  if (x >= 97 && x <= 122) || (x >= 32 && x <= 90) {
    return true;
  }
  false
}

pub fn solve() {
  // a-z: 97 - 122
  let f = File::open(FILENAME).unwrap();
  let mut reader = BufReader::new(&f);
  let mut s = String::new();
  let mut arr:Vec<i32> = Vec::new();
  for l in reader.lines() {
    s = l.unwrap();
    arr = s.split(',').map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>();
    break;
  }
  println!("{:?}", arr);
  for x in 97..123 {
    for y in 97..123 {
      for z in 97..123 {
        let mut narr: Vec<i32> = Vec::new();
        narr.push(x); narr.push(y); narr.push(z);
        let len = 100;
        let mut res: Vec<char> = Vec::new();
        let mut all_alphabet = true;
        for m in 0..len {
          let k = arr[m] ^ narr[(m % 3)];
          if !is_alphabet(k) {
            all_alphabet = false;
          }
          res.push(char::from_u32(k as u32).unwrap());
        }
        if all_alphabet {
          println!("{:?}", res);
          println!("{:?}", narr.iter().map(|x| char::from_u32(*x as u32).unwrap()).collect::<Vec<char>>());
        }
      }
    }
  }
  // manual analysis, key is "god"
  let key = ['g', 'o', 'd'];
  let mut sum = 0;
  let mut i = 0;
  for a in arr {
    let k = key[i % 3] as u32;
    sum += (a as u32) ^ k;
    i += 1;
  }
  println!("sum={}", sum);
}

