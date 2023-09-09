use parking_lot::Mutex;
use std::rc::Rc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use backoff::ExponentialBackoff;

type ArcAccount = Arc<Mutex<Account>>;

struct Account {
    balance: usize,
}

fn transfer(from: ArcAccount, to:ArcAccount, amount: usize){
    let op = || {
        if let Some(mut from) = from.try_lock(){ //dung try_lock de tra ve gia tri result, muc dich xem co lock dc hay khong
            if let Some(mut to) = to.try_lock(){
                from.balance -= amount;
                to.balance += amount;
                return Ok(());
            }
        }
        Err(0)?
    };
    let backoff = ExponentialBackoff::default();
    backoff::retry(backoff, op);
    // loop{
    //     if let Some(mut from) = from.try_lock(){ //dung try_lock de tra ve gia tri result, muc dich xem co lock dc hay khong
    //         if let Some(mut to) = to.try_lock(){
    //             from.balance -= amount;
    //             to.balance += amount;
    //             return;
    //         }
    //     }
    //     // thread::sleep(Duration::from_millis(2)); // cach1
    // }
}

fn main(){

    let transaction_1 = thread::spawn(move || {
        transfer(a, b, 100);
    });
    let transaction_2 = thread::spawn(move || {
        transfer(b, a, 300);
    });
}