use std::{io, process::exit, time::Instant};

fn fibonacci(n: u128) -> u128 {
    if n == 1 || n == 2 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

fn fibonacci_iterative(n: u128) -> u128 {
    if n <= 0 {
        return 0;
    } else if n == 1 || n == 2 {
        return 1;
    }

    let mut n2: u128 = 1;
    let mut n1: u128 = 1;
    let mut current = n1 + n2;

    for _ in 3..(n + 1) {
        current = n1 + n2;
        n2 = n1;
        n1 = current;
    }
    return current;
}

fn main() {
    println!("Please input the n-th element to calculate");
    let mut n = String::new();

    io::stdin().read_line(&mut n).expect("Insert wrong value!");

    let n: u128 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Cannot parse inserted value");
            exit(1);
        }
    };

    println!("Start testing Iterative Fibonacci calculation...");
    let start = Instant::now();
    let result = fibonacci_iterative(n);
    let duration = start.elapsed();
    println!("Iterative [duration {:?}] result: {result}.", duration);

    println!("Start testing Recursive Fibonacci calculation...");
    let start = Instant::now();
    let result = fibonacci(n);
    let duration = start.elapsed();
    println!("Recursive [duration {:?}] result: {result}.", duration);
}
