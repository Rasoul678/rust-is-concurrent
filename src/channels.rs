use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn channels() {
    let (sender, receiver) = mpsc::channel::<String>();

    let sender2 = sender.clone();

    let handle = thread::spawn(move || {
        let message_vec = "Hello From Other Channel"
            .split(" ")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        for message in message_vec {
            sender.send(message).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let message_vec = "Message From Other Thread"
            .split(" ")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        for message in message_vec {
            sender2.send(message).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for r_msg in receiver {
        println!("{}", r_msg);
    }

    handle.join().unwrap();
}
