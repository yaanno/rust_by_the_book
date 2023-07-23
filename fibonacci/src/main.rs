fn main() {
    /*
    usize.MAX is the upper bound for values on the stack
    max is 94 for the fib array length
    */
    const TARGET: usize = 20;
    let mut fib = [0; TARGET];
    for n in 0..TARGET {
        fib[n] = if n >= 2 { fib[n - 1] + fib[n - 2] } else { n };
    }
    println!("{:?}", fib);
}
