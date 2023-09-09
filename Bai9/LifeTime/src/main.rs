fn main() {
    // let num1 = 10;
    // let num2 = 30;
    // let result = get_ref(&num1, &num2);
    // println!("{}", result);
}

fn get_ref<'a>(param_1: & 'a i32, param_2: &'a i32) -> &'a i32{
    if param_1 > param_2{
        param_1
    }else{
        param_2
    }
}

#[allow(dead_code)]
fn test<'a, 'b>(param1: i32, param2: &'a str, param3: &'a str, param4: i32) -> &'a str{
    if param1 == 7 && param4 > 10 {
        param2
    }else{
        param3
    }
}