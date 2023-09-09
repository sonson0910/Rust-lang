use crossbeam_channel::unbounded;
use std::thread;


enum Message{
    PrintMsg(String),
    Sum(i32, i32),
    Quit,
}

enum MainMsg{
    ResultSum(i32),
    MainQuit,
}

fn main() {
    let (worker_tx, worker_rx) = unbounded();
    let (main_tx, main_rx) = unbounded();
    let a = thread::spawn(move || loop{
        match worker_rx.recv(){
            Ok(msg) => match msg{
                Message::PrintMsg(data) => println!("{}", data),
                Message::Sum(a, b) => {
                    main_tx.send(MainMsg::ResultSum(a + b));
                }
                Message::Quit => { 
                    println!("Quit program");
                    main_tx.send(MainMsg::MainQuit);
                    break;
                }
            },
            Err(e) => {
                println!("Error data: {}", e);
                main_tx.try_send(MainMsg::MainQuit);
                break;
            },
        }
    });

    worker_tx.send(Message::PrintMsg(("My name is Son").to_owned()));
    worker_tx.send(Message::Sum((5), (17)));
    worker_tx.send(Message::Quit);
    // drop(s); //thay cho quit

    while let Ok(msg) = main_rx.recv(){
        match msg{
            MainMsg::ResultSum(a) => println!("Main result: {}", a),
            MainMsg::MainQuit => println!("main quit"),
        }
    }

    a.join();

    // let a = thread::spawn(move || loop{
    //     match worker_rx.recv(){
    //         Ok(msg) => match msg{
    //             Message::PrintMsg(data) => println!("{}", data),
    //             Message::Sum(x, y) => println!("{} + {} = {}", x, y, x + y),
    //             Message::Quit => {
    //                 println!("Quit program");
    //                 break;
    //             }
    //         },
    //         Err(e) => {
    //             println!("Error data: {}", e);
    //             break;
    //         },
    //     }
    // });

    // worker_tx.send(Message::PrintMsg(("My name is Son").to_owned()));
    // worker_tx.send(Message::Sum((5), (17)));
    // worker_tx.send(Message::Quit);
    // // drop(s); //thay cho quit
    // a.join();

    // let a = thread::spawn(move || match r.recv(){
    //     Ok(msg) => println!("{}", msg),
    //     Err(e) => println!("{:?}", e),
    // });

    // s.send("Hello my name is Son");
    // a.join();
}
