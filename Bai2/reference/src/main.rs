fn main() {
    let mut s1 = String::from("Hello");
    let len = caculate_length(&mut s1);
    println!("Do dai cua {} la: {}", s1, len);
}

// fn caculate_length(some_string:String) -> (String, usize){
//     let length = some_string.len();
//     (some_string, length)
// }

fn caculate_length(some_string: &mut String) -> usize{
    some_string.push_str(" world");
    let length = some_string.len();
    length
}
