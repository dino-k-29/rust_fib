fn main() {
    print_fib(10);
    print_fib(18);
    print_fib(0);
}

fn print_fib(target: u32) {
    println!("Fibonacci sequence of {}:", target);
    println!("0");
    for n in 1..=target {
        println!("{}", fib(n));
    }
}

fn fib(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n-1) + fib(n-2),
    }
}
