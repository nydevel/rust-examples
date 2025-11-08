use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use std::thread;

fn main() {
    let counter = Arc::new(AtomicUsize::new(0));

    let mut handles = vec![];

    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Результат: {}", counter.load(Ordering::SeqCst)); // 10
}
