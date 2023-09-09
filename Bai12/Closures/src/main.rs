// fn add(a: i32, b: i32) -> i32{
//     a + b
// }

fn math(a: i32, b: i32, ob: Box<dyn Fn(i32, i32) -> i32>) -> i32{ // Box: vung nho heap
    ob(a, b)
}

fn main() {
    // let a = | a: i32, b: i32 | a + b;
    // println!("a: {}", a(3,5));

    // let b = |x, b, c, d| x + b - c + d;
    // println!("b: {}", b(3, 6, 8, 1));
    let add = |a, b| a + b;
    let mul = |a, b| a * b;
    let add = Box::new(add);
    let mul = Box::new(mul);
    println!("{}", math(2, 3, add));
    println!("{}", math(2, 6, mul));
}
