use iterator::Fibonacci;

fn main() {
    let fib = Fibonacci::new();

    for v in fib.take(10) {
        println!("{}", v)
    }
}
