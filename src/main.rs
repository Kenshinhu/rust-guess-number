use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the Number!");
    println!("Please input your guess!");

    let secret_number: u32 = rand::thread_rng().gen_range(1, 10);

    // println!("secret number is :{}", secret_number);

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Equal => {
                println!("You win!!");
                break;
            }
            Ordering::Greater => println!("Too Big"),
        }
    }
}
