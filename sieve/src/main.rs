use std::collections::VecDeque;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::spawn;
use std::time::Instant;
use std::vec;

fn main() {
    let now = Instant::now();
    let (tx, rx) = mpsc::channel();
    let (tx_end, rx_end) = mpsc::channel();
    spawn(|| prime_test(rx, tx_end, vec![2], VecDeque::from([2])));

    spawn(move || {
        for x in 2..100_000_000 {
            tx.send(x).unwrap();
        }
        tx.send(0).unwrap();
    });
    

    if let Ok(x) = rx_end.recv() {
        let elapsed = now.elapsed();
        println!("Elapsed before for loop: {:.2?}", elapsed);
        println!("{}", x.len());
        for i in x {
            //println!("{}", i);
        }
        let elapsed = now.elapsed();
        println!("Elapsed after for loop: {:.2?}", elapsed);
    }
}
fn prime_test(rx: Receiver<u32>, tx_end: Sender<Vec<u32>>, mut vec: Vec<u32>, mut comp_vec: VecDeque<u32>) {
    // create a channel for the next thread
    let (tx_next, rx_next) = mpsc::channel();
    // First received value: this value is the prime this agent checks with
    let p = comp_vec.pop_front().unwrap();

    // until the highest prime that is smaller than p*p is computed don't create new thread and don't send to it
    let mut i = p;
    while i < p*p{
        if let Ok(x) = rx.recv() {
            // send to next and stop if 0 was received
            if x == 0 {
                tx_end.send(vec).unwrap();
                return;
            }
            // if value is prime, store prime in vec
            else if x % p != 0 {
                vec.push(x);
                comp_vec.push_back(x);
                i = x;
            }
        }
    }
    // spawn the thread and send next_prime as first element
    spawn(|| prime_test(rx_next, tx_end, vec, comp_vec));

    // should receive all primes from previous prime_test until -1 gets send
    loop {
        if let Ok(x) = rx.recv() {
            // send to next and break if 0 was received
            if x == 0 {
                tx_next.send(0).unwrap();
                return;
            }
            // if value is prime, send to next channel
            else if x % p != 0 {
                tx_next.send(x).unwrap();
            }
        }
    }
}
