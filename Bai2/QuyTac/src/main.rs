fn main() {
    // let _s1 = String::from("Hello");
    // let _s2 = _s1.clone(); // copy
    // println!("{}", _s1);

    // let s = String::from("Hello");
    // takes_ownership(s);
    // println!("{}", s);

    let s = gives_ownership();
    println!("{}", s);

}

fn gives_ownership() -> String{
    let some_string = String::from("Hello");
    some_string
}

// fn takes_ownership(some_string: String){
//     println!("{}", some_string);
// }