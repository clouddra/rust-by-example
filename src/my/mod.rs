mod inaccessible;
pub mod nested;

pub fn function() {
    println!("my::function()");
}

fn private_function() {
    println!("private my::function()");
}

pub fn indirect_access() {
    print!("my::indirect_access");
    private_function();
}
