use core::blockchain;
use std::thread;
use std::time::Duration;
fn main() {
    let mut bc = blockchain::BlockChain::new_blockchain();
    thread::sleep(Duration::from_secs(5));
    bc.add_block("Alic -> Bob: 5 btc ".to_string());
    thread::sleep(Duration::from_secs(5));
    bc.add_block("Bob -> Cindy: 2 btc ".to_string());
    thread::sleep(Duration::from_secs(5));
    bc.add_block("Cindy -> David: 1.5 btc ".to_string());
    for b in bc.blocks{
        println!("+++++++++++++++++++++++++++++++++++++");
        println!("{:#?}", b);
        println!("+++++++++++++++++++++++++++++++++++++");
    }
}
