use std::{thread,time::Duration,sync::mpsc};

fn main(){
    let v = vec![7,2,4,3,8,1];
    let handle = thread::spawn(move || {
        for i in 0..6{
            println!("Thread Count: {}", v[i]);
            thread::sleep(Duration::from_millis(1));
        }
    });
    
    let handle2 = thread::spawn(|| {
        for i in 1..7{
            println!("Second Thread Count: {}",i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    handle2.join().unwrap();
    handle.join().unwrap();

    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();

    let handle3 = thread::spawn(move || {
        for i in 1..13{
            if i%4 == 0 {
                let msg = String::from("Hello World!");
                tx.send(msg);
            }
            thread::sleep(Duration::from_millis(10))
        }
    });
    
    let receive = rx.recv().unwrap();
    println!("Got: {}",receive);
    handle3.join().unwrap();

    let handle4 = thread::spawn(move || {
        let v = vec![
            String::from("Hello"),
            String::from("World"),
            String::from("My"),
            String::from("Name"),
            String::from("Is"),
            String::from("Ibrahim")
        ];

        for str in v{
            tx2.send(str).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for recieved in rx{
        println!("Got: {}",recieved);
    }
    handle4.join().unwrap();
}