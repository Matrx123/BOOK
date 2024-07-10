use std::{sync::mpsc, thread, time::Duration};

fn main() {
    println!("Using Message Passing to Transfer Data Between Threads");

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec!["Hi", "from", "the", "thread"];
        // let val = String::from("Hello");
        // tx.send(val).unwrap();

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        //println!("value is >>> {val}");
    });

    // let received = rx.recv().unwrap();
    // println!("Got : {received}");

    for received in rx {
        println!("Got : {received}");
    }

    cloning_the_transmitter();
}

//do this to create multiple producers
fn cloning_the_transmitter() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec!["Hi", "this", "is", "clone", "speaking"];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec!["some", "more", "messages", "for", "you"];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for receiver in rx {
        println!("Got : {receiver}");
    }
}
