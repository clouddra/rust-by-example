fn function() {
    println!("outer function");
}

mod cool {
    pub fn function() {
        println!("cool::function");
    }
}

mod my {
    fn function() {
        println!("my::function");
    }

    mod cool {
        pub fn function() {
            println!("my::cool::function");
        }
    }

    pub fn indirect_call() {
        print!("my::indirect_call ");
        self::function();
        function();

        self::cool::function();

        super::function();

        {
            use cool::function as root_function;
            root_function();
        }
    }
}

fn main() {
    my::indirect_call();
}
