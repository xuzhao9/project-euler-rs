pub struct FiboSeq {
    n_minus_1: u32,
    n_minus_2: u32,
}

impl Iterator for FiboSeq {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        let mut result = self.n_minus_1 + self.n_minus_2;
        if result == 0 {result = 1;}
        self.n_minus_2 = self.n_minus_1;
        self.n_minus_1 = result;
        Some(result)
    }
}

pub fn fibo_seq () -> FiboSeq {
    FiboSeq{n_minus_1: 0, n_minus_2: 0}
}
