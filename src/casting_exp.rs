#![allow(overflowing_literals)]

fn main() {
  let decimal = 65.4321f32;
  let integer = decimal as u8;
  let character = integer as char;
  println!("decimal: {}, integer: {}, character: {}", decimal, integer, character);

  // 1000 - 256 - 256 - 256 = 232
  println!("1000 as u8 {}", 1000 as u8);

  // -1 + 256
  println!("  -1 as a u8 is : {}", -1i8 as u8);

  // 2's complement
  println!("1000 as i8 {}", 1000 as i8);

  let x = 1u8;
  let y = 2u32;
  
  // i32 for integers, and f64 for floating-point numbers.
  let i = 1;
  let f = 1.0;
  
  println!("size of x {}", std::mem::size_of_val(&x));
  println!("size of y {}", std::mem::size_of_val(&y));
  println!("size of i {}", std::mem::size_of_val(&i));
  println!("size of f {}", std::mem::size_of_val(&f));

  let mut vec = Vec::new();
  vec.push(5);
  
  println!("{:?}", vec);

  type Nanosecond = u64;
  
  let ns = 5 as Nanosecond;
  println!("ns: {}", ns);

  let z = {
    { 2 * 10 }
  };

  println!("z: {}", z);
  
}
