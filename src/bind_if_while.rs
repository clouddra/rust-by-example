fn main() {
    println!("Tell me type of person you are");

    match age() {
        0 => println!("I'm not born"),
        n @ 1...12 => println!("age {:?}", n),
        n => println!("n {:?}", n),
    }

    if_let();
    while_let();
}

fn age() -> u32 {
    10
}

fn if_let() {
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    if let Some(i) = number {
        println!("Matched {:?}", i);
    }

    if let Some(i) = letter {
        println!("Matched {:?}!", i)
    } else {
        println!("didnt match anything");
    }

    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("Matched {:?}", i);
    } else if i_like_letters {
        println!("Didn't match a number");
    } else {
        println!("i don't like letters");
    };
}

fn while_let() {
    let mut optional = Some(0);

    while let Some(i) = optional {
        if i > 9 {
            println!("> 9");
            optional = None;
        } else {
            println!("i = {:?}", i);
            optional = Some(i + 1);

        }
    }
}
