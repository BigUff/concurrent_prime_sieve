use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::spawn;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let (tx, rx) = mpsc::channel();
    let (tx_end, rx_end) = mpsc::channel();
    spawn(|| prime_test(rx, tx_end));

    spawn(move || {
        for x in 2..100_000 {
            tx.send(x).unwrap();
        }
        tx.send(-1).unwrap();
    });
    loop {
        if let Ok(p) = rx_end.recv() {
            if p == -1 {
                break;
            }
            //println!("{}", p);
        }
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
/*
fn prime_test(rx: Receiver<i32>, tx_collect: Sender<i32>, counter: i32) {
    let counter = counter + 1;
    let mut prime_checker = 0;
    let (mut tx_next, rx_next) = mpsc::channel();
    if counter > 100 {
        tx_next = tx_collect;
    } else {
        spawn(move || prime_test(rx_next, tx_collect, counter));
    }

    loop {
        match rx.recv() {
            Ok(k) => {
                if prime_checker == 0 {
                    prime_checker = k;
                } else {
                    if k % prime_checker != 0 {
                        if k > 100 {
                            tx_next.send(-1).unwrap();
                            break;
                        }
                        tx_next.send(k).unwrap();
                    }
                }
            }
            Err(_) => todo!(),
        }
    }
}
*/
fn prime_test(rx: Receiver<i32>, tx_end: Sender<i32>) {
    // create a channel for the next thread
    let (tx_next, rx_next) = mpsc::channel();
    // First received value: this value is the prime this agent checks with
    let p;
    if let Ok(x) = rx.recv() {
        if x == -1 {
            tx_end.send(-1).unwrap();
            return;
        } else {
            p = x;
            tx_end.send(p).unwrap();
            spawn(move || prime_test(rx_next, tx_end));
        }
    } else {
        panic!("failed on receive");
    }
    // should receive all primes from previous prime_test until -1 gets send
    loop {
        if let Ok(x) = rx.recv() {
            // send to next and break if -1 was received
            if x == -1 {
                tx_next.send(-1).unwrap();
                break;
            }
            // if value is prime, send to next channel
            else if x % p != 0 {
                tx_next.send(x).unwrap();
            } else {
            }
        }
    }
}
