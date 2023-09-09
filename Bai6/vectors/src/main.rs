fn main() {
    // Vectors
    let a = [1, 2, 3];
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let mut v2 = Vec::new();
    v2.push(1);
    v2.push(2);
    v2.push(3);
    v2.push(4);
    v2.push(5);

    println!("{:?}", v2);

    // let four = &v[20];
    // println!("{}", four);

    match v.get(3){
        Some(four) => println!("This is four element = {}", four),
        None => println!("This is not a four element"),
    }

    for i in &mut v{
        *i += 10;
    }

    for i in &v {
        println!("{}", i)
    }
}
