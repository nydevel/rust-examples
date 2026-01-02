fn main() {
    let mut x = String::from("hi");
    let r1 = &mut x; // одна мутабельная ссылка
    let r2 = r1; // не копия, а перенос ссылки (move), r1 больше нельзя использовать

    *r2 = String::from("hello");
    println!("{}", x); // 15
}
