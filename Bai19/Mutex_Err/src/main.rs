use parking_lot::{Mutex, ReentrantMutex};
use std::rc::Rc;

type Data = Rc<ReentrantMutex<i32>>; // relock khi goi de quy

fn recurse(data: Data, id: i32) -> usize {
    let mut locked = data.lock();
    match id {
        id if id == 0 => 0,
        id => recurse(Rc::clone(&data), id + 1),
    }
}

fn main() {
    println!("Hello, world!");
}
