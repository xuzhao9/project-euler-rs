use std::collections::HashSet;
extern crate primes;
fn get_primes(p: &mut Vec<u32>, n: u32) {
    let mut i = 2;
    let mut b_arr = Vec::<bool>::new();
    for _ in (0.. n) {
        b_arr.push(true);
    }
    b_arr[0] = false;
    b_arr[1] = false;
    loop {
        if i * i >= n {
            break;
        } else {
            if b_arr[i as usize] {
                let mut j = i * 2;
                loop {
                    b_arr[j as usize] = false;
                    j = j + i;
                    if j >= n {
                        break;
                    }
                }
            }
            i = i + 1;
        }
    }
    for i in (0..b_arr.len()) {
        if b_arr[i] {
            p.push(i as u32);
        }
    }
}


pub fn solve() {
    let num = 5000;
    let mut primes = Vec::<u32>::new();
    let module = 1e16 as u32;
    get_primes(&mut primes, num);
    let sum: u32 = primes.iter().fold(0, |x, acc| x + acc);
    let mut re = Vec::<u32>::with_capacity((sum+6) as usize);
    for _ in (0..sum+6) {
        re.push(0);
    }
    println!("resize:{}", re.len());
    re[0] = 1;
    re[1] = 0;
    let mut s = 0;
    let mut set = HashSet::new();
    set.insert(0);
    for &j in &primes {
        let mut t_set = HashSet::new();
        for &x in set.iter() {
            re[(x + j)as usize] += 1;
            t_set.insert(x + j);
        }
        for &x in t_set.iter() {
            set.insert(x);
        }
    }
    println!("{:?}", set);
    let mut res = 0;
    for r in (0..re.len()) {
        if primes::is_prime(r as u64) {
            res += re[r];
            res =  res % module;
        }
    }
    println!("{}", res);
}
