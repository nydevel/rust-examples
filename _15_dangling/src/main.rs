fn dangle() -> &String {
    // Компилятор запретит
    let s = String::from("hello");
    &s // s уничтожится при выходе из fn
}

fn main() {
    let r = dangle(); // Ошибка: dangling reference
}
