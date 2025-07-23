// TASK:
// Implement the following types, which meet conditions:

//     OnlySync is Sync, but !Send.
//     OnlySend is Send, but !Sync.
//     SyncAndSend is both Sync and Send.
//     NotSyncNotSend is both !Sync and !Send.

// All inner details of implementation are on your choice.

// Play with these types from multiple threads to see how compile time fearless concurrency works in practice.

#![feature(negative_impls)]

use core::time;
use std::{
    sync::mpsc::{self, channel, Sender},
    thread,
};

#[derive(Debug)]
struct OnlySync(u8);
impl !Send for OnlySync {}

#[derive(Debug)]

struct OnlySend(u8);
impl !Sync for OnlySend {}

// no need to implement, because Sync and Send traits are automatically implemented in this case
#[derive(Debug)]
struct SyncAndSend(u8);

#[derive(Debug)]
struct NotSyncNotSend(u8);
impl !Send for NotSyncNotSend {}
impl !Sync for NotSyncNotSend {}

fn main() {
    let (tx, rx) = mpsc::channel();
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();
    //let (tx3, rx3) = mpsc::channel();
    let a: u8 = 1;
    let b = SyncAndSend(2);
    let ccc = OnlySend(3);
    let not_sync_not_send = NotSyncNotSend(4);
    //let d = OnlySync(4);
    //let e = NotSyncNotSend(5);
    let handle = thread::spawn(move || {
        tx.send(a);
        thread::sleep(time::Duration::from_millis(1));
        tx1.send(b);
        thread::sleep(time::Duration::from_millis(1));
        tx2.send(ccc);
        thread::sleep(time::Duration::from_millis(1));
       // Can not be sent safely, Trait Send in not implemented. compilation error
       // tx3.send(not_sync_not_send);
    });

    let handle2 = thread::spawn(move || {
        let a = rx.recv().unwrap();
        let b = rx1.recv().unwrap();
        let ccc = rx2.recv().unwrap();
        //let c = rx1.recv().unwrap();
        println!("a is {a}");
        println!("b is {:?}", b);
        println!("ccc is {:?}", ccc);
    });

    let handles = vec![handle, handle2];
    for handle in handles {handle.join().unwrap()};
}
