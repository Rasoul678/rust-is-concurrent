use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

pub fn dead_lock_mutex() {
    println!("{}", "=".repeat(30));
    println!("Dead Locks Mutex");
    println!("{}", "=".repeat(30));

    let m1 = Arc::new(Mutex::new(0));
    let m2 = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    {
        let m1_ref = Arc::clone(&m1);
        let m2_ref = Arc::clone(&m2);

        let handle_one = thread::spawn(move || {
            println!("Thread 1");
            let mut m_one = m1_ref.lock().unwrap();
            *m_one += 1;

            println!("Thread 1 holds m1 lock and starts waiting m2 lock");

            let delay = Duration::from_secs(1);
            thread::sleep(delay);

            // ? Dead lock work around
            if let Ok(ref mut m_two) = m2_ref.try_lock() {
                **m_two += 1;
            }

            // ? Dead lock
            // let mut m_two = m2_ref.lock().unwrap();
            // *m_two += 1;
        });

        handles.push(handle_one);
    }

    {
        let m1_ref = Arc::clone(&m1);
        let m2_ref = Arc::clone(&m2);

        let handle_two = thread::spawn(move || {
            println!("Thread 2");
            let mut m_two = m2_ref.lock().unwrap();
            *m_two += 1;

            println!("Thread 2 holds m2 lock and starts waiting m1 lock");

            let mut m_one = m1_ref.lock().unwrap();
            *m_one += 1;
        });

        handles.push(handle_two);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("m1: {}", *m1.lock().unwrap());
    println!("m2: {}", *m2.lock().unwrap());
}
