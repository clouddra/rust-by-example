#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
struct Point {
    x: f32,
    y: f32,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    fn area(&self) -> f32 {
        let &Rectangle { p1: Point { x: x1, y: y1 }, p2: Point { x: x2, y: y2 } } = self;
        ((x1 - x2) * (y1 - y2)).abs()
    }
}

fn square(p: Point, h: f32) -> Rectangle {
    Rectangle {
        p1: p,
        p2: Point {
            x: p.x + h,
            y: p.y + h,
        },
    }
}

enum Person {
    Engineer,
    Scientist,
    Height(i32),
    Weight(i32),
    Info { name: String, height: i32 },
}

fn inspect(p: Person) {
    match p {
        Person::Engineer => println!("Is an engineer!"),
        Person::Scientist => println!("Is a scientist!"),
        Person::Height(i) => println!("Height {}", i),
        Person::Weight(i) => println!("Weight {}", i),
        Person::Info { name, height } => println!("{} is {} tall!", name, height),
    }
}

#[derive(Debug)]
enum Status {
    Rich,
    Poor,
}

enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

// use List;

#[derive(Debug)]
enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        Nil
    }
    
    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }
    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }
}
use List::{Cons, Nil};

static LANGUAGE: &'static str = "RUST";
const THRESHOLD: i32 = 10;

fn main() {
    let rectangle = Rectangle {
        p1: Point { x: 0.3, y: 0.4 },
        p2: Point { x: 5.0, y: 2.0 },
    };
    println!("Rectangle {:?}", rectangle);
    println!("Rectangle area: {}", rectangle.area());
    println!("Square {:?}", square(Point { x: 1.0, y: 1.0 }, 2.0));

    let dave = Person::Info {
        name: "Dave".to_owned(),
        height: 72,
    };
    inspect(dave);

    let person = Person::Weight(10);
    inspect(person);
    use Status::{Poor, Rich};
    println!("Poor, Debug: {:?}, Int: {:}", Poor, Poor as i32);
    println!("roses are #{:06x}", Color::Green as i32);
    
    
    let mut list = List::new();
    list = list.prepend(1);
    list = list.prepend(2);
    println!("list {:?}", list);
    println!("list length: {}", list.len());

    println!("language: {}", LANGUAGE);
    println!("Threshold: {}", THRESHOLD);
}
