fn main() {
    let mut s = String::from("Hello");
    let _r1 = &mut s;
    let _r2 = &mut s;
    println!("{} {}", _r1, _r2);
}

fn okmain(){
    let s = String::from("hello");
    let _r1 = &s;
    let _r2 = &s;
    println!("{} {}", _r1,  _r2);
}
