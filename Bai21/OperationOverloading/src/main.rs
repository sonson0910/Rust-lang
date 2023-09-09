use std::ops::Add;

#[derive(Debug)]
struct Number(i32);

impl Add<i32> for Number{
    type Output = Self;

    fn add(self, input: i32) -> Self::Output{
        Number(self.0 + input)
    }
}

fn main() {
    println!("{:?}", Number(5) + 10);
}
