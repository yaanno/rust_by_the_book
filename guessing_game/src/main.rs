use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let max_rounds = 10;
    let mut round = 1;
    println!("Guess the number!");
    println!("-----------------");
    loop {
        if round == max_rounds {
            println!("The number was {}", secret_number);
            println!("Sorry, Game Over...");
            break;
        }
        println!("Round {}, GO!", round);
        println!("Please input your number:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You got it!");
                break;
            },
        }
        round = round + 1;
    }
}
