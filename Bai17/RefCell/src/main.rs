use std::cell::{RefCell, Ref};
#[derive(Debug)]

struct Channel {
    name: RefCell<String>,
}

fn main() {
    let mychannel = Channel{
        name: RefCell::new("Lap tring Blocchain".to_owned()),
    };

    {
        let mut a = mychannel.name.borrow_mut();
        *a = "Lap trinh Blockchain".to_owned();
    }
    {mychannel.name.replace("Lap trinh Blockchain :))".to_owned());}

    // println!("a: {}", a);
    println!("my channel: {:?}", mychannel.name);
}
