// Полиморфизм в Rust через трейты

// 1. Статический полиморфизм (compile-time) через generics
trait Drawable {
    fn draw(&self);
}

struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing a circle with radius {}", self.radius);
    }
}

impl Drawable for Rectangle {
    fn draw(&self) {
        println!("Drawing a rectangle {}x{}", self.width, self.height);
    }
}

// Статический полиморфизм - тип известен на этапе компиляции
fn draw_static<T: Drawable>(shape: &T) {
    shape.draw();
}

// 2. Динамический полиморфизм (runtime) через trait objects
fn draw_dynamic(shape: &dyn Drawable) {
    shape.draw();
}

// 3. Пример с возвратом trait object
fn create_shape(is_circle: bool) -> Box<dyn Drawable> {
    if is_circle {
        Box::new(Circle { radius: 5.0 })
    } else {
        Box::new(Rectangle {
            width: 10.0,
            height: 20.0,
        })
    }
}

// 4. Пример с несколькими трейтами
trait Animal {
    fn make_sound(&self) -> String;
    fn name(&self) -> &str;
}

struct Dog {
    name: String,
}

struct Cat {
    name: String,
}

impl Animal for Dog {
    fn make_sound(&self) -> String {
        "Woof!".to_string()
    }

    fn name(&self) -> &str {
        &self.name
    }
}

impl Animal for Cat {
    fn make_sound(&self) -> String {
        "Meow!".to_string()
    }

    fn name(&self) -> &str {
        &self.name
    }
}

fn main() {
    println!("=== Статический полиморфизм ===");
    let circle = Circle { radius: 3.0 };
    let rectangle = Rectangle {
        width: 5.0,
        height: 7.0,
    };

    draw_static(&circle);
    draw_static(&rectangle);

    println!("\n=== Динамический полиморфизм ===");
    draw_dynamic(&circle);
    draw_dynamic(&rectangle);

    println!("\n=== Коллекция trait objects ===");
    let shapes: Vec<Box<dyn Drawable>> = vec![
        Box::new(Circle { radius: 2.5 }),
        Box::new(Rectangle {
            width: 3.0,
            height: 4.0,
        }),
        Box::new(Circle { radius: 1.0 }),
    ];

    for shape in &shapes {
        shape.draw();
    }

    println!("\n=== Динамическое создание объектов ===");
    let shape1 = create_shape(true);
    let shape2 = create_shape(false);
    shape1.draw();
    shape2.draw();

    println!("\n=== Животные ===");
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog {
            name: "Buddy".to_string(),
        }),
        Box::new(Cat {
            name: "Whiskers".to_string(),
        }),
        Box::new(Dog {
            name: "Max".to_string(),
        }),
    ];

    for animal in &animals {
        println!("{} says: {}", animal.name(), animal.make_sound());
    }
}
