use std::collections::HashMap;

fn main() {
    // let mu = String::from("MU");
    // let mc = String::from("MC");

    // let mut scores = HashMap::new();

    // scores.insert(mu, 9);
    // scores.insert(mc, 10);

    // let team_name = String::from("MC");
    // let score = scores.get(&team_name);

    // for (key, value) in &scores{
    //     println!("{} {}", key, value)
    // }
    // println!("score = {:?}", score);

    // let mut scores = HashMap::new();

    // scores.insert(String::from("MC"), 9);
    // scores.insert(String::from("MC"), 8);

    // scores.entry(String::from("MU")).or_insert(5);
    // scores.entry(String::from("MU")).or_insert(1);
    // println!("scores = {:?}", scores);

    let text = "hello world this is wonderful world";
    let mut map = HashMap::new();
    for i in text.split_whitespace(){
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
