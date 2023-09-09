// #[derive(Debug)]

// enum ClassType{
//     Good,
//     Bad,
// }

// #[derive(Debug)]
// struct Class {
//     name: String,
//     kind: ClassType,
// }

// fn main(){
//     let classA = Class{
//         name: "A15".to_owned(),
//         kind: ClassType::Good,
//     };
//     dbg!(&classA);
// }

// fn main() {
//     let a = 1;
//     let b = 2;
//     assert_eq!(a, b, "Values should not be equal");
// }

fn main(){
    let a = "Hello";
    let b = "World";
    let result: String = format!("{} {}", a, b);
    println!("{}", result);
}

