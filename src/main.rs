use rand::Rng;
use std::io;
fn main() {
    println!("Guess the secret");

    let secret = rand::thread_rng().gen_range(0..100);
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Read Failure");
        let guess: u128 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        //println!("secret = {secret}");
        println!("your guess {guess}");
        match guess.cmp(&secret) {
            std::cmp::Ordering::Less => println!("Too small"),
            std::cmp::Ordering::Equal => {
                println!("You win");
                break;
            }
            std::cmp::Ordering::Greater => println!("Too large"),
        }
    }
}
