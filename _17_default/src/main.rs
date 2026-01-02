#[derive(Default, Debug)] // Авто-derive для полей с Default
struct Config {
    timeout: u64, // 0 по умолчанию
    name: String, // "" по умолчанию
}

fn main() {
    let config: Config = Default::default(); // Все поля по умолчанию
    let custom = Config {
        timeout: 30,
        ..Default::default() // Остальное по умолчанию
    };
}
