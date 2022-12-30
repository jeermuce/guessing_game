use rand::Rng;
use std::io::{self, stdout, Write};
const MAX_ATTEMPTS: u32 = 5; // maximum number of attempts allowed
fn main() {
    let language =
        get_response("Choose a language: English or Spanish\nElige un idioma: Inglés o Español");
    if language == "english" || language == "inglés" || language == "en" || language == "i" {
        english();
    } else if language == "spanish" || language == "español" || language == "es" || language == "s"
    {
        spanish();
    } else {
        println!("Invalid language. Please enter a valid language\nIdioma inválido. Por favor ingrese un idioma válido");
    }
}

fn english() {
    let mut secret = rand::thread_rng().gen_range(1..100);
    let mut attempts = 0; // counter for the number of attempts made
    loop {
        println!("{secret}");
        let guess: u32 = get_guess("Guess: ");
        if guess == 0 {
            print!("\x1B[2J\x1B[1;1H");
            println!("must be non zero integer");
            continue;
        }
        if attempts > MAX_ATTEMPTS - 1 {
            // check if the limit has been reached
            println!("You lose. The secret was {secret}");
            break;
        }
        attempts += 1; // increment the counter

        match guess.cmp(&secret) {
            std::cmp::Ordering::Less => {
                println!("{guess} is too small");
                println!("You have {} attempts left", MAX_ATTEMPTS - attempts);
                continue;
            }
            std::cmp::Ordering::Equal => {
                println!("You win! {guess} is the correct answer!");
                let response = get_response("Continue? y/n");
                if response == "n" || response == "no" {
                    println!("Thanks for playing!");
                    print!("\x1B[2J\x1B[1;1H");
                    break;
                } else if response == "y" || response == "yes" {
                    secret = rand::thread_rng().gen_range(1..100); // generate a new secret number
                    print!("\x1B[2J\x1B[1;1H");
                    continue;
                } else {
                    println!("Invalid response. Please enter a valid response.")
                }
            }
            std::cmp::Ordering::Greater => {
                println!("{guess} is too large");
                println!("You have {} attempts left", MAX_ATTEMPTS - attempts);
                continue;
            }
        }
    }
}
fn spanish() {
    let mut secret = rand::thread_rng().gen_range(1..100);
    let mut attempts = 0; // counter for the number of attempts made
    loop {
        println!("{secret}");
        let guess: u32 = get_guess("Adivina: ");
        if guess == 0 {
            print!("\x1B[2J\x1B[1;1H");
            println!("debe ser un entero no cero");
            continue;
        }
        if attempts > MAX_ATTEMPTS - 1 {
            // check if the limit has been reached
            println!("Perdiste. El secreto era {secret}");
            break;
        }
        attempts += 1; // increment the counter

        match guess.cmp(&secret) {
            std::cmp::Ordering::Less => {
                println!("{guess} es demasiado pequeño");
                println!("Tienes {} intentos restantes", MAX_ATTEMPTS - attempts);
                continue;
            }
            std::cmp::Ordering::Equal => {
                println!("¡Ganaste! {guess} es la respuesta correcta!");
                let response = get_response("Continuar? s/n");
                if response == "n" || response == "no" {
                    println!("Thanks for playing!");
                    print!("\x1B[2J\x1B[1;1H");
                    break;
                } else if response == "s" || response == "si" {
                    secret = rand::thread_rng().gen_range(1..100); // generate a new secret number
                    print!("\x1B[2J\x1B[1;1H");
                    continue;
                } else {
                    println!("Invalid response. Please enter a valid response.")
                }
            }
            std::cmp::Ordering::Greater => {
                println!("{guess} es demasiado grande");
                println!("Tienes {} intentos restantes", MAX_ATTEMPTS - attempts);
                continue;
            }
        }
    }
}

fn get_guess(prompt: &str) -> u32 {
    print!("{prompt}");
    stdout().flush().expect("Flush failure");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Read Failure");
    let input: u32 = input.trim().parse().unwrap_or(0);
    input
}

fn get_response(prompt: &str) -> String {
    println!("{prompt}");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_lowercase()
}
