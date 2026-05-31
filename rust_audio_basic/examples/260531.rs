// 260531 多執行緒共享計數器
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let counter_clone = Arc::clone(&counter);

    let handle = thread::spawn(move || {
        for _ in 0..5 {
            thread::sleep(Duration::from_millis(500));
            println!("counter_clone: {}", counter_clone.lock().unwrap());
        }
    });

    {
        thread::sleep(Duration::from_millis(1000));

        let mut data = counter.lock().unwrap();
        *data = 42;
    }

    handle.join().unwrap();
}
