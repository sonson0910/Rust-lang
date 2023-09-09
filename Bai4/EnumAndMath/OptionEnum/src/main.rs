fn main() {
    // let _number = Some(5);
    // let _string = Some("a String");

    // let _nonenumber: Option<i32> = None;
    let x = 5;
    let y = Some(5);
    let sum = x + y.unwrap_or(1); // unwap: ddi ve i32
    println!("sum = {}", sum);
}
