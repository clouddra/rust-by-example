fn main() {
    let x = 10;
    fn function(i: i32) -> i32 {
        i + 1
    }
    println!("function {}", function(1));

    let closure_annotated = |i: i32| -> i32 { i * x + 1 };
    println!("clousure annotated {}", closure_annotated(2));

    let closure_inferred = |i| i * x + 1;
    println!("clousure annotated {}", closure_inferred(3));

    let one = || 1;
    println!("one closure {}", one());

    capturing();
    input_param();
}

fn capturing() {
    use std::mem;

    let color = "green";

    let print = || println!("'color': {}", color);
    print();
    print();

    let mut count = 0;
    let mut inc = || {
        count += 1;
        println!("mut count, {}", count);
    };

    inc();
    inc();

    // 2nd borrow
    // let reborrow = &mut count;

    let movable = Box::new(3);

    let consume = || {
        println!("movable: {:?}", movable);
        mem::drop(movable)
    };

    consume();

    // 2nd move
    // consume();
}

fn apply<F>(f: F)
    where F: FnOnce(){
    f();
}

fn apply_to_3<F>(f: F) -> i32
    where F: Fn(i32) -> i32
{
    f(3)
}

fn input_param() {
    use std::mem;

    let greeting = "hello";
    let mut farewell = "goodbye".to_owned();

    let diary = || {
        println!("I said {}.", greeting);
        farewell.push_str("!!!");
        println!("farewell {}", farewell);
        mem::drop(farewell);
    };

    apply(diary);
    // diary();

    let double = |x| 2 * x;
    println!("3 double {}", apply_to_3(double));

}
