fn main() {
    let s = String::from("hello world");
    let slice: &str = &s[0..5]; // "hello"
    let literal: &str = "static"; // 'static lifetime
    fn takes_slice(s: &str) {
        println!("{}", s.len());
    } // Принимает оба
    takes_slice("lit");
    takes_slice(&s);
}
