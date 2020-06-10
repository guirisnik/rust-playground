use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    println!("Input your guess:");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    for _attempt_number in (1..11).rev() {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => { println!("You win!"); break; },
            Ordering::Greater => println!("Too big!"),
        }
    }
}
