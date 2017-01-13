fn eat_box_i32(boxed_i32: Box<i32>) {
    println!("eat_box_i32 = {}", boxed_i32);
}

fn borrow_i32(borrowed_i32: &i32) {
    println!("borrow_i32 = {}", borrowed_i32);
}

fn main() {
    borrow();
    mutability();
    freezing();
    aliasing();
}

fn borrow() {
    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;

    borrow_i32(&boxed_i32);
    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);
    borrow_i32(&stacked_i32);

    {
        let _ref_to_i32 = &boxed_i32;
        // still have existing reference in outer scope, cannot take ownership
        // eat_box_i32(boxed_i32);
    }

    eat_box_i32(boxed_i32);
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book {
    author: &'static str,
    title: &'static str,
    year: u32,
}

// immutable reference (borrow)
fn borrow_book(book: &Book) {
    println!("Immutably borrowed {} - {} edition", book.title, book.year);
}

// mutable reference (borrow)
fn new_edition(book: &mut Book) {
    book.year = 2014;
    println!("Mutably borrows {} - {} edition", book.title, book.year);
}

fn mutability() {
    let immutbook = Book {
        author: "author A",
        title: "title A",
        year: 1979,
    };

    let mut mutbook = immutbook;

    borrow_book(&immutbook);
    borrow_book(&mutbook);

    new_edition(&mut mutbook);

    // make mutable reference from immutable object
    // new_edition(&mut immutbook);
}

fn freezing() {
    let mut _mutable_int = 7i32;
    {
        let _borrow_int = &_mutable_int;

        // frozen while borrowed
        // _mutable_int = 50;
    }

    _mutable_int = 3;
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn aliasing() {
    let mut point = Point { x: 0, y: 0, z: 0 };
    {
        let borrowed_point = &point;
        let another_borrow = &point;

        println!("Point = ({}, {}, {})",
                 borrowed_point.x,
                 another_borrow.y,
                 point.z);

        // currently borrowed as immutable, cannot mutable borrow
        // let mutable_borrow = &mut point;
    }

    {
        let mutable_borrow = &mut point;
        mutable_borrow.x = 5;
        mutable_borrow.y = 2;
        mutable_borrow.z = 1;

        // currently borrowed as mutable, cannot immutable borrow
        // let y = &point.y;

        println!("{:?}", mutable_borrow);
    }

    // can borrow since outside scope
    let borrowed_point = &point;
    println!("{:?}", borrowed_point);
}
