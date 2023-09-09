fn main() {
    let s1 = gives_ownership();
    let s2 = String::from("Hello");
    let s3 = takes_and_gives_back(s2);
    println!("{}, {}", s1, s3);
}

fn gives_ownership() -> String{
    let some_string = String::from("Hello");
    some_string
}

fn takes_and_gives_back(some_string: String) -> String{
    some_string
}
