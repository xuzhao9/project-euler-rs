fn first_fac(n: u64) -> u64 {
    if n % 2 == 0 {
        return 2;
    } else {
        let m = ((n as f64).sqrt()).ceil() as u64;
        for i in (3..m+1) {
            if i % 2 == 0 {
                continue;
            }
            if n % i == 0 {
                return i;
            }
        }
        return n;
    }
}

fn factors(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::<u64>::new();
    loop {
        let f = first_fac(n);
        factors.push(f);
        if f == n {
            break;
        } else {
            n = n / f;
        }
    }
    factors
}

pub fn solve() -> u64 {
    let number = 600851475143;
    let factors = factors(number);
    let max = *(factors.iter().max().unwrap());
    println!("{}", max);
    max
}
