use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let mut n: i64 = input.trim().parse().expect("Invalid input");

    let mut a: [i64; 9] = [500000, 200000, 100000, 50000, 20000, 10000, 5000, 2000, 1000];

    let mut t:i64 = 0;
    for x in &a{
        t += n / x;
        n %= x;
        if n <= 0 {
            break
        }
    }
    if n == 0 {
        println!("{}", t)
    }else{
        println!("-1")
    }
}
