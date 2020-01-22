use std::sync::mpsc::*;
use std::sync::mpsc::{sync_channel};


fn main() -> std::io::Result<()> {
    let (sync_sender, receiver) = sync_channel(10000);
    sync_sender.send(1).unwrap();
    let msg = receiver.recv().unwrap();
    println!("{:?}", msg);
    Ok(())
}
