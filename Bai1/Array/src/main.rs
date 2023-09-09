fn main() {
    let number = [100, 200, 300];
    let get_number = number[1];
    println!("number = {}", get_number);
    let _hashing = [0; 32];
    println!("hashing = {:?}", _hashing);
    for i in _hashing.iter(){ // iter: lay ra tung item trong hashing
        print!("{} ", i)

    }
}
