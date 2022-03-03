fn main() {
    println!("{}", ok::foo());
}

mod ok {

    pub fn foo() -> &'static str {
        "I'm foo!"
    }
}
