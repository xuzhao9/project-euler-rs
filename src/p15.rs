pub fn solve() {
    let num = 22;
    let mut f = Vec::<Vec<u64>>::new();
    for i in (0..num) {
        f.push(Vec::<u64>::new());
        for _ in (0..num) {
            f[i].push(0);
        }
    }
    for i in (0.. num) {
        for j in (0..num) {
            if i == 0 && j == 0 {
                f[i][j] = 1;
            } else {
                if i == 0 {
                    f[i][j] = f[i][j-1];
                } else if j == 0 {
                    f[i][j] = f[i-1][j];
                } else {
                    f[i][j] = f[i-1][j] + f[i][j-1];
                }
            }
        }
    }
    println!("{}", f[20][20]);
}
