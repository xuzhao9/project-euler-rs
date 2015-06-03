fn palin_p(n: u64) -> bool {
    let s = n.to_string().into_bytes();
    let mut sr = s.clone();
    sr.reverse();
    for i in 0..s.len() {
        if s[i] != sr[i] {
            return false;
        }
    }
    return true;
}

pub fn solve() {
    let mut v: Vec<u64> = Vec::new();
    let mut max = 0;
    for i in (100..1000) {
        for j in (i..1000) {
            let k = i * j;
            if palin_p(k) {
                if max < k {
                    max = k;
                }
            }
        }
    }
    println!("{}", max);
}
