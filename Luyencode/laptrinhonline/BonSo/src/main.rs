// use std::collections::HashMap;
// use std::{io, vec};

// #[derive(Debug)]
// struct ToaDo {
//     x: i32,
//     y: i32,
// }

// fn main() {
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).expect("Failed to read input");

//     let n: i32 = input.trim().parse().expect("Invalid input");
//     let mut a: HashMap<i32, i32> = HashMap::new();
//     input.clear();
//     io::stdin().read_line(&mut input).expect("Failed to read input");
//     let mut c = 0;
//     for num_str in input.trim().split_whitespace() {
//         let num: i32 = num_str.parse().expect("Invalid input");
//         a.insert(c, num);
//         c+=1;
//     }
//     let mut sumtmp: HashMap<i32, Vec<ToaDo>> = HashMap::new();
//     let mut count = 0;
//     for i in 0..n - 1 {
//         for j in i + 1..n {
//             let t = ToaDo{
//                 x: i + 1,
//                 y: j + 1,
//             };
//             match (a.get(&i), a.get(&j)){
//                 (Some(inputx), Some(inputy)) => {
//                     let sum = inputx + inputy;
//                     if let Some(tt) = sumtmp.get_mut(&sum) {
//                         tt.push(t);
                        
//                     } else {
//                         sumtmp.insert(sum, vec![t]);
//                     }
                    
//                     // println!("x = {}, y = {}", inputx, inputy);
//                 }
//                 _ => return 
//             }
            
//         }
        
//     }

//     for h in sumtmp{
//         let vec: Vec<ToaDo> = h.1;
//         let len = vec.len();
//         if len > 1 {
//             for i in 0..len - 1 {
//                 for j in i + 1.. len{
//                     if vec[i].y < vec[j].x {
//                         count+=1
//                     }
//                 }
//             }
//         }
//     }
//     println!("{}", count);
//     // println!("{:#?}", sumtmp);
// }


use std::collections::HashMap;
use std::io;

#[derive(Debug)]
struct ToaDo {
    x: i32,
    y: i32,
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let n: i32 = input.trim().parse().expect("Invalid input");
    
    let mut a: Vec<i128> = Vec::new();
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    a.extend(input.trim().split_whitespace().map(|num_str| num_str.parse::<i128>().unwrap()));
    
    let mut count = 0;
    let mut sumtmp: HashMap<i128, Vec<ToaDo>> = HashMap::new();

    for i in 0..n - 1 {
        for j in i + 1..n {
            let t = ToaDo{
                x: i + 1,
                y: j + 1,
            };
            
            let sum = a[i as usize] + a[j as usize];
            let entry = sumtmp.entry(sum).or_insert(Vec::new());
            entry.push(t);
        }
    }

    for vec in sumtmp.values() {
        let len = vec.len();
        if len > 1 {
            for i in 0..len - 1 {
                for j in i + 1..len {
                    if vec[i].y < vec[j].x {
                        count += 1
                    }
                }
            }
        }
    }
    // for (index, vec) in sumtmp.values().enumerate(){
    //     let len = vec.len();
    //     if len > 1 {
    //         for j in index + 1..len{

    //         }
    //     }
    // }

    println!("{}", count);
}


// use std::collections::HashMap;
// use std::io;

// fn main() {
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).expect("Failed to read input");

//     let n: usize = input.trim().parse().expect("Invalid input");

//     let mut a: Vec<i64> = Vec::new();
//     input.clear();
//     io::stdin().read_line(&mut input).expect("Failed to read input");
//     a.extend(input.trim().split_whitespace().map(|num_str| num_str.parse::<i64>().unwrap()));

//     let mut count = 0;
//     let mut sum_count: HashMap<i64, i64> = HashMap::new();

//     for i in 0..n - 1 {
//         for j in i + 1..n {
//             let sum = a[i] + a[j];
//             *sum_count.entry(sum).or_insert(0) += 1;
//         }
//     }

//     for freq in sum_count.values() {
//         count += freq * (freq - 1) / 2;
//     }

//     println!("{}", count);
// }
