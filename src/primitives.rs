// use std::fmt;

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (integer, boolean) = pair;
    (boolean, integer)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn transpose(matrix: Matrix) -> Matrix {
    let Matrix(m0_0, m0_1, m1_0, m1_1) = matrix;
    Matrix(m0_0, m1_0, m0_1, m1_1)
}

impl std::fmt::Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        try!(write!(f, "({} {})\n", self.0, self.1));
        write!(f, "({} {})", self.2, self.3)

    }
}

fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    println!("millon {}", 1_000_000u32);

    let long_tuple = (1u8, 2u16, 3);
    println!("{}, {}, {}", long_tuple.0, long_tuple.1, long_tuple.2);

    let tuple_of_tuples = ((10, 20, 30), long_tuple, (40));
    println!("{:?}", tuple_of_tuples);

    let pair = (1, true);
    println!("reverse {:?}", reverse(pair));

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("Matrix\n{}", matrix);
    println!("Transpose Matrix\n{}", transpose(matrix));

    let ys: [i32; 500] = [0; 500];
    analyze_slice(&ys[1..4]);
    println!("array {:?}", ys[1]);
}
