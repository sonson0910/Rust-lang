fn main() {
    let _value = Some(5);
    match _value{
        Some(5) => println!("Bang 5"),
        _ => println!("Khac 5"),
    }
}
