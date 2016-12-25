fn main() {
  let integer = 1u32;
  let _boolean = true;

  let mut mut_binding = 2;
  println!("An mut integer {}", mut_binding);
  mut_binding = 1;
  let copied_integer = integer;
  
  println!("An integer {}", copied_integer);
  println!("An mut integer after {}", mut_binding);
  
  let x = 1;
  println!("Outer {}", x);
  {
    let x = 10;
    println!("Inner {}", x);
  }
  
  let x = "rust";
  println!("Outer {}", x);
}
