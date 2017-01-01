struct Val {
  val: f64
}

struct GenVal<T> {
  gen_val: T
}

impl Val {
  fn value(&self) -> &f64 {
  &self.val
  }
}

impl <T> GenVal<T> {
  fn value(&self) -> &T { &self.gen_val }
}

fn main() {
  let x = Val { val: 3.6 };
  let y = GenVal { gen_val: 3};

  println!("{} {}", x.value(), y.value());
  
  traits();
}


struct Empty;
struct Null;


trait DoubleDrop<T> {
    fn double_drop(self, _: T);
}

// parameter T, caller U
impl<T,U> DoubleDrop<T> for U {
  // moved both T and U
  fn double_drop(self, _: T) {}
} 

fn traits() {
  let empty = Empty;
  let null = Null;

  empty.double_drop(null);
  
  // cannot move twice 
  // empty;
  // null;
}
