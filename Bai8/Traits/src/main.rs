struct Data{
    num1: i32,
    num2: i32,
    str1: String,
    optional: Option<i32>,
}

struct Data2{
    num1: i32,
    num2: i32,
    str1: String,
    optional: Option<i32>,
}

impl Data{
    fn new() -> Self{
        Data { 
            num1: 15,
            num2: 25,
            str1: "Some string 2".to_string(),
            optional: None,
        }
    }
}

trait Transform{
    fn revert(&self) -> String{
        String::from("No string...")
    }

    fn output_revert(&self){
        println!("{}", self.revert())
    }
}

impl Transform for Data{
    // fn revert(&self) -> String {
    //     self.str1.chars().rev().collect()::<String>()
    // }
}

impl Transform for Data2{
    fn revert(&self) -> String{
        (self.num1 + self.num2).to_string()
    }
}

fn main() {
    let a = Data::new();

    let b = Data2{
        num1:10,
        num2:20,
        str1: String::from("ok string"),
        optional: None,
    };
    a.output_revert();

    println!("{}", b.revert());
}
