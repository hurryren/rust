use std::sync::mpsc;
use std::sync::{Mutex,Arc};
use std::thread;
use std::time::Duration;
use std::rc::Rc;


fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        
        });
        handles.push(handle);
    }

    for handle in handles{
        println!("num: {:?}",counter);
        handle.join().unwrap();
    }
}

fn mute_test1() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}

fn thread_test() {
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("tx1 hi"),
            String::from("tx1 from"),
            String::from("tx1 the"),
            String::from("tx1 thread"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {:?}", received);
    }

    // handle.join().unwrap();
}
