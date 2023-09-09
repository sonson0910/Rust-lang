use std::cmp::{PartialOrd, Ordering};

#[derive(PartialEq)]
struct User{
    id: i32,
    name: String,
}

impl PartialOrd for User{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>{
        Some(self.name.cmp(&other.name))
    }
}

fn main() {
    let a = User {
        id : 1, 
        name: "A".to_owned(),
    };
    let b = User {
        id : 2, 
        name: "B".to_owned(),
    };
    let c = a.partial_cmp(&b);
    println!("{:?}", c);
}
