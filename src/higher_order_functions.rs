fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));
    println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    println!("2 in array1 {}", array1.iter().any(|&x| x == 2));
    println!("2 in array2 {}", array2.into_iter().any(|&x| x == 2));

    let mut iter = vec1.iter();
    println!("find 2 in vec 1 {:?}", iter.find(|&&x| x == 2));

    let mut into_iter = vec![4, 5, 6].into_iter();
    println!("find 2 in vec 2 {:?}", into_iter.find(|&x| x == 2));

    println!("2 in [1,2,3] {:?}", [1, 2, 3].iter().find(|&&x| x == 2));
    println!("2 in [4,5,6] {:?}", [4, 5, 6].iter().find(|&&x| x == 2));

    hof();
}

fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

fn hof() {
    println!("Sum of all squared odd numbers under 1000");
    let upper = 1000;

    let mut acc = 0;

    for n in 0.. {
        let n_sq = n * n;
        if n_sq >= upper {
            break;
        } else if is_odd(n_sq) {
            acc += n_sq;
        }
    }

    let sum_of_squared_odd: u32 = (0..)
        .map(|n| n * n)
        .take_while(|&n| n < upper)
        .filter(|&n| is_odd(n))
        .fold(0, |sum, i| sum + i);
    println!("functional {}", sum_of_squared_odd);

    println!("Imperative style: {}", acc);
}
