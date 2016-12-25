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
}
