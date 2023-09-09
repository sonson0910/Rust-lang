use std::io;

fn get_inputStr() -> Option<String>{
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err(){
        println!("Please try again");
    }
    let input = buffer.trim().to_owned();
    if &input == "" {
        None
    }else{
        Some(input)
    }
}

fn get_input() -> Option<i32>{
    let input = match get_inputStr(){
        Some(input) => input,
        None => return None,
    };

    let parsed_input: Result<i32,_> = input.parse();
    match parsed_input{
        Ok(input) => Some(input),
        Err(_) => None,
    }
}

fn main() {
    let n = match get_input(){
        Some(number) => number,
        None => return,
    };
    for i in 1..=n{
        if (i % 2 != 0){
            print!("{}", i)
        } else{
            if (i % 2 == 0){
                print!("L")
            } 
            if (i % 4 == 0){
                print!("T")
            }
            if (i % 8 == 0){
                print!("O")
            }
            if (i % 16 == 0) {
                print!("L")
            }
        }
        println!();
    }
}
