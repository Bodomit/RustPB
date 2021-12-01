use std::io;

fn main() {
    println!("Enter n:");

    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");

    let n: u32 = n.trim().parse().expect("Please enter a positive interger.");
    let x = fib(n);

    println!("Fib({}) = {}", n, x);
}

fn fib(n: u32) -> u32 {
    let mut x0 = 0;
    let mut x1 = 1;

    if n == 0 {
        return 0;
    }

    if n == 1 {
        return 1;
    }

    for _ in 1..n {
        let x2 = x1 + x0;
        x0 = x1;
        x1 = x2;
    }

    x1
}
