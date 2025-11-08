use std::sync::atomic::{AtomicU32, Ordering};

fn main() {
    let counter = AtomicU32::new(0);

    counter.store(40, Ordering::SeqCst);

    println!("Значение: {}", counter.load(Ordering::SeqCst))
}
