fn main() {
    let n = 5;

    if n < 0 {
        print!("negative {}", n);
    } else if n > 0 {
        print!("positive {}", n);
    } else {
        print!("zero");
    }

    let big_n = if n < 10 && n > -10 {
        println!(", is a small number");
        10 * n
    } else {
        println!(", is a big number");
        n / 2
    };

    println!("{} -> {}", n, big_n);

    let mut count = 0u32;
    loop {
        count += 1;
        if count == 3 {
            println!("continue 3");
            continue;
        }
        println!("{}", count);
        if count == 5 {
            println!("break 5");
            break;
        }
    }

    'outer: loop {
        println!("outer");
        'inner: loop {
            println!("inner");
            // break;
            break 'outer;
        }
        println!("after outer");
    }
    println!("exited loop");


    let mut n = 1;

    while n < 0 {
        if n % 15 == 0 {
            println!("15 fizzbuzz");
        } else if n % 3 == 0 {
            println!("3 fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n)
        };
        n += 1;
    }

    for n in 1..101 {
        if n % 15 == 0 {
            println!("{} divides 15 fizzbuzz", n);
        } else if n % 3 == 0 {
            println!("{} divides 3 fizz", n)
        } else if n % 5 == 0 {
            println!("{} 5 buzz", n);
        } else {
            println!("{}", n);
        }

    }
}
