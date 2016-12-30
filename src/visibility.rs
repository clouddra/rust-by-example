mod my {
    fn private_function() {
        println!("private function");
    }

    pub fn pub_function() {
        private_function();
        println!("pub function");
    }

    pub mod nested {
        pub fn public_mod_fn() {
            private_mod_fn();
            println!("called `my::nested::public_mod_fn()`");
        }

        fn private_mod_fn() {
            println!("called `my::nested::private_mod_fn()`");
        }
    }

    pub struct WhiteBox<T> {
        pub contents: T,
    }

    #[allow(dead_code)]
    pub struct BlackBox<T> {
        contents: T,
    }

    impl<T> BlackBox<T> {
        pub fn new(contents: T) -> BlackBox<T> {
            BlackBox { contents: contents }
        }
    }
}

fn main() {
    // private
    // my::private_function();

    my::pub_function();
    my::nested::public_mod_fn();
    // private
    //  my::nested::private_mod_fn();

    let white_box = my::WhiteBox { contents: "public info" };
    println!("white box contains {}", white_box.contents);

    // private
    // let black_box = my::BlackBox { contents: "classified information" };

    let _black_box = my::BlackBox::new("classified info");
    // private fields
    // println!("black box contents {}", _black_box.contents);
}
