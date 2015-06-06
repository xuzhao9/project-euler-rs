fn steps(f: &Vec<u64>, i: u64) -> u64 {
    let mut steps = 0;
    let mut x = i;
    loop {
        if x == 1 {break;}
        if f.len() > x as usize {
            steps += f[x as usize];
            break;
        }
        if x % 2 == 0 {
            x = x / 2;
            steps += 1;
        } else {
            x = 3 * x + 1;
            steps += 1;
        }
    }
    return steps;
}

pub fn solve() {
    let target = 1000001;
    let mut max = 0;
    let mut num = 0;
    let mut f = Vec::<u64>::new();
    f.push(0);
    for i in (1..target) {
        let r = steps(&f, i);
        if r > max {
            max = r;
            num = i;
        }
        f.push(r);
    }
    println!("{}", num);
}
