use std::mem;

fn main() {
    let name = "Joaquim";
    let yob: u16 = 1999;

    let cool_numbers: [u32; 3] = [7, 22, 69];

    println!("Hey {}, you're {} years old aren't you?", name, calc_age(yob));
    println!();
    println!("{:?} - {}", cool_numbers, mem::size_of_val(&cool_numbers));

    for i in 1..=25 {
        println!("i: {}", i);
    }

    let pair: (i8, i8) = (4, -6);
    match pair {
        (x, y) if x == y => println!("Equal"),
        (x, y) if x + y == 0 => println!("Equal Zero"),
        (x, _) if x % 2 == 0 => println!("X is even"),
        _ => println!("No match"),
    }
}

fn calc_age(yob: u16) -> u16 {
    2021 - yob
}
