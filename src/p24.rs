fn fac(k: u32) -> u32 {
    if k == 0 {return 1;}
    k * fac(k - 1)
}

fn digit(n: u32, i: u32) -> u32 {
    assert!(i >= 1);
    n / fac(i - 1)
}

pub fn solve() {
    let mut num = 1e6 as u32 - 1;
    let mut res = Vec::<u32>::new();
    let mut b = Vec::<bool>::new();
    for _ in (0..10) {
        b.push(true);
    }
    let mut i = 10;
    loop {
        let k = digit(num, i);
        let mut j = 0;
        let mut r = 0;
        for i in (0..10) {
            if b[i] && (j == k) {
                r = i as u32;
                b[i] = false;
                break;
            }
            if b[i] == false {
                continue;
            } else {
                j += 1;
            }
        }
        res.push(r);
        num = num % fac(i - 1);
        if i == 1 {
            break;
        } else {
            i -= 1;
        }
    }
    println!("{:?}", res);
}
