mod my;

fn function() {
    println!("function()");
}

fn main() {
    my::function();

    function();

    my::indirect_access();

    my::nested::function();

}
