fn main() {
    raii();
    ownership();
    mutability();
}

fn create_box() {
    let _box1 = Box::new(3i32);
}

fn raii() {
    let _box2 = Box::new(5i32);

    {
        let _box3 = Box::new(4i32);
    }

    for _ in 0u32..1_000 {
        create_box();
    }
}

fn destroy_box(c: Box<i32>) {
    println!("Destroying a box that contains {}", c);
}

fn ownership() {
    let x = 5u32;

    // copy
    let y = x;


    println!("x = {}, y = {}", x, y);


    let a = Box::new(5i32);
    println!("a = {}", a);

    // move, onwership changed
    let b = a;

    // lost ownership
    // println!("a = {}", a);

    // move
    destroy_box(b);

    // lost ownership
    // println!("b = {}", b);
}

fn mutability() {
    let immutable_box = Box::new(5u32);

    println!("immutable_box = {}", immutable_box);

    // move and change ownership, mutability
    let mut mutable_box = immutable_box;
    // println!("immutable_box after = {}", immutable_box);

    println!("mutable_box = {}", mutable_box);

    *mutable_box = 4;

    println!("mutable_box after = {}", mutable_box);

}
