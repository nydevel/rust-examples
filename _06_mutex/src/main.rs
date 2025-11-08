use std::sync::{Arc, Mutex};
use std::thread;

// Изменяемый через мютекс счетчик

fn main() {
    let counter = Arc::new(Mutex::new(0));


    let counter_clone = Arc::clone(&counter);

    // Замыкание ЗАБИРАЕТ владение counter_clone
    let handler = thread::spawn(move || {
        let mut num = counter_clone.lock().unwrap();

        *num +=2;
    });

    handler.join().unwrap();
    println!("{}", *counter.lock().unwrap());
}
