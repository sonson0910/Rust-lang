fn main() {
    let tup = ("Hello", 100_000);
    println!("{:?}", tup);
    let ( _string, _integer ) = tup;
    let _integer = tup.1;
    println!("{}", _integer);
    println!("{}", _string);
}
