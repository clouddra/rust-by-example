// extern crate rand;

// use std::io;
// use std::cmp::Ordering;
// use rand::Rng;
// }

use std::fmt;

#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}


impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let List(ref vec) = *self;
        try!(write!(f, "["));

        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                try!(write!(f, ", "));
            }
            try!(write!(f, "{}", v));
        }

        write!(f, "]")


    }
}

struct City {
    name: &'static str,
    lat: f32,
    lon: f32,
}

impl fmt::Display for City {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        write!(f,
               "{}, {:.3} {}, {:.3} {}",
               self.name,
               self.lat.abs(),
               lat_c,
               self.lon.abs(),
               lon_c)
    }
}

struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "RGB ({0:}, {1:}, {2:}) 0x{0:02X}{1:02X}{2:02X}",
               self.red,
               self.green,
               self.blue)
    }
}


fn main() {
    println!("{} days", 31);
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    println!("{subject} {verb} {object}",
             object = "the lazy dog",
             subject = "the quick brown fox",
             verb = "jumps over");
    println!("{1:?} {0} is the {actor:?} name.",
             Structure(3),
             "Christian",
             actor = "actor's");

    let range = MinMax(10, 60);
    println!("MinMax {}", range);
    println!("Complex {}",
             Complex {
                 real: 3.3,
                 imag: 7.2,
             });
    println!("List {}", List(vec![1, 2, 3]));

    for city in [City {
                     name: "Dublin",
                     lat: 53.46464,
                     lon: -6.25758573,
                 },
                 City {
                     name: "Oslo",
                     lat: 56.5,
                     lon: 10.67,
                 }]
        .iter() {
        println!("{}", *city);
    }

    println!("{}",
             Color {
                 red: 0,
                 green: 3,
                 blue: 254,
             });
}
