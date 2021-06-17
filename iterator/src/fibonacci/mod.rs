pub struct Fibonacci {
    n: u64,
}

impl Fibonacci {
    pub fn new() -> Fibonacci {
        Fibonacci { n: 0 }
    }

    pub fn fib(n: u64) -> u64 {
        match n {
            0 => 0,
            1 => 1,
            _ => Fibonacci::fib(n - 2) + Fibonacci::fib(n - 1),
        }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        self.n = self.n + 1;

        Some(Fibonacci::fib(self.n))
    }
}
