// Author: Xu Zhao (i@xuzhao.net)
// Date: Aug 30, 2016

//! problem 42

use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::io::BufReader;
use std::env;

fn word_to_num(word: &str) -> i32 {
  let mut w = 0i32;
  for b in word.chars() {
    let n = b as u8 - 'A' as u8 + 1;
    w = w + n as i32;
  }
  return w;
}

pub fn solve() {
  let mut path = env::home_dir().unwrap();
  path.push("p042_words.txt");
  let file = match File::open(path.as_path()) {
    Err(why) => panic!("couldn't open {}: {}", path.display(), why.description()),
    Ok(file) => file,
  };
  let reader = BufReader::new(file);
  let mut cnt = 0;
  for line_raw in reader.lines() {
    let line = line_raw.ok().unwrap().replace("\"", "");
    let words: Vec<&str> = line.split(",").collect();
    for w in words.iter() {
      let c = word_to_num(w) * 2 * 4 + 1;
      let s = (c as f64).sqrt() as i32;
      if s * s == c {
        cnt += 1;
      }
    }
  }
  println!("{}", cnt);
}

