pub fn concurrent_rust() {
    use std::thread;
    use std::time::Duration;

    let spawn = "spawned";
    println!("Concurrent Rust!");

    let handle = thread::spawn(move || {
        for i in 0..10 {
            println!("Number {i} in {spawn} thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 0..5 {
        println!("Number {i} in main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}
