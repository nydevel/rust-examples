use std::cell::Cell;

fn main() {
    let x = Cell::new(5);
    println!("x: {}", x.get());
    x.set(10);
    println!("x: {}", x.get());
}
