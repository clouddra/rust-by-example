fn main() {
    let i = 3;
    {
        let ref borrow1 = i;
        println!("borrow1: {}", borrow1);
    }

    {
        let ref borrow2 = i;
        println!("borrow2: {}", borrow2);
    }

    explicit_annotation();
    functions();
    methods();
    structs();
}

fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x: {}, y: {}", x, y);
}

fn failed_borrow<'a>() {
    let _x = 12;
}

fn explicit_annotation() {
    let (four, nine) = (4, 9);

    print_refs(&four, &nine);

    failed_borrow();
}

fn print_one<'a>(x: &'a i32) {
    println!("print_one: {}", x);
}

fn add_one<'a>(x: &'a mut i32) {
    *x += 1;
}

fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("print multi: {}, {}", x, y);
}

// fn invalid_output<'a>() -> &'a i32 {
//    &7 // lifetime shorter than a' (a' lives longer than the function)
// }

fn pass_x<'a, 'b>(x: &'a i32, _: &'b i32) -> &'a i32 {
    x
}

fn functions() {
    let x = 7;
    let y = 9;

    print_one(&x);
    print_multi(&x, &y);

    let z = pass_x(&x, &y);

    print_one(z);
    println!("z: {}", *z);

    let mut t = 3;
    add_one(&mut t);
    print_one(&t);
}

struct Owner(i32);

impl Owner {
    fn add_one<'a>(&'a mut self) {
        self.0 += 1
    }
    fn print<'a>(&'a self) {
        println!("self.0: {}", self.0);
    }
}

fn methods() {
    let mut owner = Owner(18);

    owner.add_one();
    owner.print();
}

#[derive(Debug)]
struct Borrowed<'a>(&'a i32);

#[derive(Debug)]
struct NamedBorrowed<'a> {
    x: &'a i32,
    y: &'a i32,
}

#[derive(Debug)]
enum Either<'a> {
    Num(i32),
    Ref(&'a i32),
}

fn structs() {
    let x = 18;
    let y = 15;

    let single = Borrowed(&x);
    let double = NamedBorrowed { x: &x, y: &y };
    let reference = Either::Ref(&x);
    let number = Either::Num(y);

    println!("Borrowed x: {:?}", single);
    println!("Borrowed x, y: {:?}", double);
    println!("Borrowed x (either): {:?}", reference);
    println!("Not borrowed (either): {:?}", number);

}
