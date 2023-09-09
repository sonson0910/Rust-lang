fn main() {
    let five = Some(5);
    let six = plus_one(five);
    println!("six = {:#?}", six);

    let none = plus_one(None);
    println!("none = {:#?}", none);
}


fn plus_one(x: Option<i32>) -> Option<i32>{
    match x{
        Some(x) => Some(x + 1),
        _ => None,
    }
}