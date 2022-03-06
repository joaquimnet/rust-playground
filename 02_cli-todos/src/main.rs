// Tutorial:
// https://www.freecodecamp.org/news/how-to-build-a-to-do-app-with-rust/

// next is: How to Insert and Save Data with a Custom Type

fn main() {
    let action = std::env::args().nth(1).expect("Please specify an action");
    let item = std::env::args().nth(2).expect("Please specify an item");

    println!("{:?}, {:?}", action, item);
}
