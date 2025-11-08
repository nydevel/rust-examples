use std::sync::{Arc, Mutex, RwLock};
use std::thread;

// Изменяемый через RWLock счетчик
// прим.: parking_lot::RwLock - быстрее стандартного RWLock

fn main() {
    let counter = Arc::new(RwLock::new(0));


    let counter_clone = Arc::clone(&counter);

    // Замыкание ЗАБИРАЕТ владение counter_clone
    let handler = thread::spawn(move || {
        let mut num = counter_clone.write().unwrap();

        *num +=3;
    });

    handler.join().unwrap();
    println!("{}", *counter.read().unwrap());
}
