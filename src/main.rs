use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let the_answer: u32 = rand::thread_rng().gen_range(1..=100);

    println!("Guess the number! [q to quit]");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        if ["q", "quit", "exit"].contains(&&*guess.trim().to_lowercase()) {
            println!("Bye!");
            break;
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}.", guess);

        match guess.cmp(&the_answer) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
