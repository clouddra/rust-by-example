fn main() {
    struct Foo {
        x: (u32, u32),
        y: u32,
    }

    let foo = Foo { x: (1, 2), y: 3 };
    let Foo { y, x: (a, b) } = foo;

    println!("a = {}, b = {}, c = {}", a, b, y);

    let Foo { y, .. } = foo;
    println!("y = {}", y);

    let pair = (2, -2);

    match pair {
        (x, y) if x == y => println!("These are twins"),
        (x, y) if x + y == 0 => println!("x + y == 0"),
        (x, _) if x % 2 == 1 => println!("Odd"),
        _ => println!("No correlation"),
    }
}
