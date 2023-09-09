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

fn get_inputInt() -> Option<i32>{
    // let input = match get_inputStr(){
    //     Some(input) => input,
    //     None => return None,
    // };

    // let parsed_input: Result<i32,_> = input.parse();
    // match parsed_input{
    //     Ok(input) => Some(input),
    //     Err(_) => None,
    // }
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok()?;
    input.trim().parse().ok()
}

fn abs(a: i32, b: i32) -> i32 {
    (a - b).abs()
}

fn main() {
    let n = match get_inputInt(){
        Some(number) => number,
        None => return,
    };
    // println!("n = {}", n);
    let mut kq: Vec<i32> = Vec::new();
    
    let mut day = String::new();
    io::stdin().read_line(&mut day).expect("Failed to read line");

    let mut dayxl: Vec<i32> = day.trim().split_whitespace().map(|x| x.parse::<i32>().expect("Invalid input")).collect();

    let mut max_num = dayxl[0];
    let mut min_num = max_num.clone();
    let mut c = 0;
    for i in 1..n{
        // let range = 0..i;
        // let max_num = dayxl[range].iter().max().cloned();
        c += 1;
 
        let h = dayxl[c];

        max_num = max_num.max(h);
        min_num = min_num.min(h);
        
        let m = abs(max_num, h);
        let q = abs(min_num, h);
        kq.push(m.max(q));
    }

    for num in kq{
        print!("{} ", num);
    }
    println!();
    
}
