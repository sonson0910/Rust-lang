use std::thread;
use std::thread::JoinHandle;
use std::time::{self, Duration};

fn Thread1() -> bool{
    print!("Hello, ");
    true
}

fn Thread2() -> bool{
    println!("my name is ");
    true
}

fn Thread3() -> bool{
    println!("Nguyen Hong Son");
    true
}

fn main() {
    // let iterations = 10;
    // let a:JoinHandle<bool> = thread::spawn(move || { // move: move toan bo vao thread
    //     thread::sleep(Duration::from_secs(1));
    //     Thread1()
    // });

    // println!("waiting for data");
    // match a.join(){
    //     Ok(value) => println!("Value: {}", value),
    //     Err(e) => println!("error: {:?}", e),
    // }

    let a:JoinHandle<bool> = thread::spawn(move || { // move: move toan bo vao thread
        thread::sleep(Duration::from_secs(1));
        Thread1()
    });

// let iterations = 10;
    let x: JoinHandle<bool> = thread::spawn(move || { // move: move toan bo vao thread
        thread::sleep(Duration::from_secs(1));
        Thread2()
    });

    
    let y: JoinHandle<bool> = thread::spawn(move || { // move: move toan bo vao thread
        thread::sleep(Duration::from_secs(1));
        Thread3()
    });

    match a.join(){
        Ok(value) => println!(""),
        Err(e) => println!("error: {:?}", e),
    }
    match x.join(){
        Ok(value) => println!(""),
        Err(e) => println!("error: {:?}", e),
    }
    match y.join(){
        Ok(value) => println!(""),
        Err(e) => println!("error: {:?}", e),
    }

}
