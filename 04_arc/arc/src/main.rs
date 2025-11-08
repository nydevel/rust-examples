use std::sync::Arc;
use std::thread;

// Данные внутри Arc неизменяемы, если не использовать паттерны внутренней изменяемости как Mutex или RwLock
// позволяет нескольким частям вашего кода разделять владение этим значением потокобезопасным способом.

fn main() {
    let data = Arc::new(vec![1,4,3]);

    // Clone - Это очень дешевая операция. Оба Arc указывают на один и тот же vec![1, 2, 3] в памяти.
    let data_clone = Arc::clone(&data);

    let handle = thread::spawn(move || {
        println!("{:?}", data_clone)
    });

    let data_clone = Arc::clone(&data);
    let handle = thread::spawn(move || {
        println!("{:?}", data_clone)
    });

    handle.join().unwrap()
}
