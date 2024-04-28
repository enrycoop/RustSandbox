fn fibonacci(n: i32) -> i32 {
    if n == 1 || n == 2 {
        1
    } else {
        fibonacci(n-1) + fibonacci(n-2)
    }
}

fn main() {
    let f = fibonacci(12);
    println!("The fibonacci number is {f}");
}