use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
fn main() {
    // let handle = thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("hi number {} from spawned thread!", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });
    // for i in 1..5 {
    //     println!("hi number {} from main thread!", i);
    //     thread::sleep(Duration::from_millis(1));
    // }
    // handle.join().unwrap();
    //移动所有权
    // let v = vec![1, 2, 3, 4];
    // let handle = thread::spawn(move || println!("{:?}", v));
    // handle.join().unwrap()

    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("I"),
            String::from("am"),
            String::from("from"),
            String::from("china"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });
    thread::spawn(move || {
        let vals = vec![
            String::from("1:where"),
            String::from("1:are"),
            String::from("1:you"),
            String::from("1:from"),
            String::from("1:?"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });
    for recived in rx {
        println!("Got:{}", recived);
    }

    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num += 1;
    }
    println!("m={:?}", m);

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for i in 1..10 {
        let count = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = count.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("result:{}", *counter.lock().unwrap());
}
