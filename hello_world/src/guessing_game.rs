use std::io;
use rand::Rng;
use std::cmp::Ordering;

const MAX_VALUE: u8 = 100;
const MIN_VALUE: u8 = 1;

fn main() {
    let the_answer: u8 = rand::thread_rng().gen_range(MIN_VALUE..=MAX_VALUE);

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

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => if num > MAX_VALUE { continue; } else { num },
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
