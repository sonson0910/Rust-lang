// use std::{io, collections::HashMap, ops::Add};

// fn main() {
//     let mut input = String::new();
//     io::stdin().read_line(&mut input).expect("Failed to read input");

//     let n: i32 = input.trim().parse().expect("Invalid input");

//     let mut a: Vec<i128> = Vec::new();
//     input.clear();
//     io::stdin().read_line(&mut input).expect("Failed to read input");
//     a.extend(input.trim().split_whitespace().map(|num_str| num_str.parse::<i128>().unwrap()));

//     let mut map: HashMap<i128, i32> = HashMap::new();

//     let mut max:i128 = 0;
//     let mut min= std::i128::MAX;
//     for  i in 0..n {      
//         let tmp: &mut i32 = map.entry(a[i as usize]).or_insert(0); 
//         *tmp += 1;
//         let tmp_num: i128 = a[i as usize];
//         if max < tmp_num{
//             max = tmp_num
//         }
//         if min > tmp_num{
//             min = tmp_num
//         }
//     }
//     if max == min{
//         println!("{}", match (map.get(&max)) {
//             Some(x) => x,
//             _ => &0
//         })
//     }else{
//         println!("{}", match (map.get(&max), map.get(&min)) {
//             (Some(x), Some(y)) => x + y,
//             _ => 0
//         });

//     }
    
// }


use std::{io, collections::HashMap};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let n: i32 = input.trim().parse().expect("Invalid input");

    let mut a: Vec<i128> = Vec::new();
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    a.extend(input.trim().split_whitespace().map(|num_str| num_str.parse::<i128>().unwrap()));

    let mut map: HashMap<i128, i32> = HashMap::new();

    let mut max:i128 = 0;
    let mut min= std::i128::MAX;
    for &num in &a {      
        let tmp = map.entry(num).or_insert(0); 
        *tmp += 1;
        
        max = max.max(num);
        min = min.min(num);
    }
    
    if max == min {
        println!("{}", map.get(&max).unwrap_or(&0));
    } else {
        let sum = map.get(&max).unwrap_or(&0) + map.get(&min).unwrap_or(&0);
        println!("{}", sum);
    }
}
