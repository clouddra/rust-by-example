pub fn public_function() {
    println!("public_function()");
}

fn private_function() {
    println!("private_function()");
}

pub fn indirect_access() {
    print!("rary indirect_access(), that \n");
    private_function();
}
