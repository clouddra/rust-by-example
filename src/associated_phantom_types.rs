use std::fmt::Debug;

trait PrintInOption {
    fn print_in_option(&self);
}

impl<T> PrintInOption for T
    where Option<T>: Debug,
          T: Debug
{
    fn print_in_option(&self) {
        println!("{:?}", Some(self));
    }
}

fn main() {
    let vec = vec![1, 2, 3];
    vec.print_in_option();
    vec.print_in_option();

    assoc_items();
    phantom();
    unit_clarification();
}

struct Container(i32, i32);

trait Contains {
    type A;
    type B;

    fn contains(&self, &Self::A, &Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains for Container {
    type A = i32;
    type B = i32;

    fn contains(&self, number_1: &Self::A, number_2: &Self::B) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }

    fn first(&self) -> Self::A {
        self.0
    }

    fn last(&self) -> Self::B {
        self.1
    }
}

fn difference<C: Contains>(container: &C) -> i32 {
    container.last() - container.first()
}

fn assoc_items() {
    let number_1 = 1;
    let number_2 = 10;

    let container = Container(number_1, number_2);

    println!("Contains {} and {}?: {}",
             &number_1,
             &number_2,
             container.contains(&number_1, &number_2));
    println!("first {}", container.first());
    println!("last {}", container.last());

    println!("diff is {}", difference(&container));
}

use std::marker::PhantomData;

#[derive(PartialEq)]
struct PhantomTuple<A, B>(A, PhantomData<B>);

#[derive(PartialEq)]
struct PhantomStruct<A, B> {
    first: A,
    phantom: PhantomData<B>,
}

fn phantom() {
    let _tuple1: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
    let _tuple2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);

    // different phantom types
    // println!("tuple1 == tuple2:, {}", _tuple1 == _tuple2);

    let _struct1: PhantomStruct<char, f32> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };

    let _struct2: PhantomStruct<char, f64> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };

    // println!("struct1 == struct2:, {}", _struct1 == _struct2);
}

use std::ops::Add;

#[derive(Debug, Clone, Copy)]
struct Inch;

#[derive(Debug, Clone, Copy)]
struct Mm;

#[derive(Debug, Clone, Copy)]
struct Length<Unit>(f64, PhantomData<Unit>);

impl<Unit> Add for Length<Unit> {
    type Output = Length<Unit>;

    fn add(self, rhs: Self::Output) -> Self::Output {
        Length(self.0 + rhs.0, PhantomData)
    }
}

fn unit_clarification() {
    let one_foot: Length<Inch> = Length(12.0, PhantomData);
    let one_meter: Length<Mm> = Length(1000.0, PhantomData);

    println!("2 feet: {:?}", one_foot + one_foot);
    println!("2 meters: {:?}", one_meter + one_meter);

    // type mismatch
    // let _one_feter = one_foot + one_meter;
}
