use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::spawn;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let (tx, rx) = mpsc::channel();
    let (tx_end, rx_end) = mpsc::channel();
    spawn(|| prime_test(rx, tx_end, vec![]));

    spawn(move || {
        for x in 2..100_000 {
            tx.send(x).unwrap();
        }
        tx.send(-1).unwrap();
    });
    

    if let Ok(x) = rx_end.recv() {
        println!("{}", x.len());
        for i in x{
            
        }
        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);
    }
    
    
}
fn prime_test(rx: Receiver<i32>, tx_end: Sender<Vec<i32>>, mut vec: Vec<i32>) {
    // create a channel for the next thread
    let (tx_next, rx_next) = mpsc::channel();
    // First received value: this value is the prime this agent checks with
    let p;
    let next_prime;
    if let Ok(x) = rx.recv() {
        if x == -1 {
            tx_end.send(vec).unwrap();
            return;
        } else {
            p = x;
        }
    } else {
        panic!("failed on receive");
    }
    // store the 2nd element to initialize next thread
    if let Ok(x) = rx.recv() {
        if x == -1 {
            tx_end.send(vec).unwrap();
            return;
        } else {
            next_prime = x;
            vec.push(next_prime);
        }
    } else {
        panic!("failed on receive");
    }

    // until p*p primes are computed don't create new thread and send to it
    for _ in p..p*p{
        if let Ok(x) = rx.recv() {
            // send to next and stop if -1 was received
            if x == -1 {
                tx_end.send(vec).unwrap();
                return;
            }
            // if value is prime, store prime in vec
            else if x % p != 0 {
                vec.push(x);
            }
        }
    }
    // spawn the thread and send next_prime as first element
    spawn(|| prime_test(rx_next, tx_end, vec));
    tx_next.send(next_prime).unwrap();

    // should receive all primes from previous prime_test until -1 gets send
    loop {
        if let Ok(x) = rx.recv() {
            // send to next and break if -1 was received
            if x == -1 {
                tx_next.send(-1).unwrap();
                return;
            }
            // if value is prime, send to next channel
            else if x % p != 0 {
                tx_next.send(x).unwrap();
            }
        }
    }
}
