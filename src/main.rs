fn main() {
    let car = String::from("Car");

    println!("The word car is {} characters long.", car.len());

    let mut mutable_car = String::from("Mutable Car");

    println!("The string Mutable Car is {} characters long.", mutable_car.len());

    mutable_car.push_str("!!!");

    println!("The string Mutable Car!!! is {} characters long.", mutable_car.len());

    println!("First word: {}", first_word(&mutable_car));
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
