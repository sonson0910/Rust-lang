#[derive(PartialEq, PartialOrd)] // ap dung cho so sanh bang, hon
// Eq: bang, Ord:hon

enum Employee{
    Marketer(i32),
    Developer(i32),
    Saler,
}

fn main() {
    let a = Employee::Marketer(60);
    let b = Employee::Developer(40);
    let c = Employee::Saler;
    if a > b {
        println!("true");
    }else{
        println!("false");
    }
}
