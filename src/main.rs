//I made this, but I want an alternative to termion and clear().expect("failed to clear screen");
use clearscreen::clear;
use rand::Rng;
use std::io::{self, stdout, Write};
const MAX_ATTEMPTS: u32 = 10; // maximum number of attempts allowed
const MIN_RANGE: u32 = 1;
const MAX_RANGE: u32 = 100;
fn main() {
    language();
}
fn language() {
    loop {
        let language = get_response(
            "Choose a language: English or Spanish\nElige un idioma: Inglés o Español",
        );
        let valid_english: Vec<String> = vec![
            "english".to_string(),
            "eng".to_string(),
            "inglés".to_string(),
            "ingles".to_string(),
            "ing".to_string(),
            "en".to_string(),
            "i".to_string(),
            "in".to_string(),
        ];
        let valid_spanish = vec![
            "spanish".to_string(),
            "spa".to_string(),
            "español".to_string(),
            "espanol".to_string(),
            "esp".to_string(),
            "es".to_string(),
            "s".to_string(),
            "sp".to_string(),
        ];
        if valid_english.contains(&language) {
            english();
            continue;
        } else if valid_spanish.contains(&language) {
            spanish();
            continue;
        } else {
            println!("Invalid language. Please enter a valid language\nIdioma inválido. Por favor ingrese un idioma válido");
        }
    }
}

fn english() {
    println!("You have chosen English");
    clear().expect("failed to clear screen");

    let mut secret = rand::thread_rng().gen_range(MIN_RANGE..=MAX_RANGE);
    let mut attempts = 0; // counter for the number of attempts made
    println!("Guess the secret number!");

    loop {
        //println!("{secret}");
        let guess: u32 = get_guess("Guess: ");
        if guess == 0 {
            clear().expect("failed to clear screen");

            println!("must be non zero integer");
            continue;
        }
        if attempts > MAX_ATTEMPTS - 1 {
            // check if the limit has been reached
            println!("You lose. The secret was {secret}");
            clear().expect("failed to clear screen");

            break;
        }
        attempts += 1; // increment the counter

        match guess.cmp(&secret) {
            std::cmp::Ordering::Less => {
                clear().expect("failed to clear screen");

                println!("{guess} is too small");
                println!(
                    "You have {}/{} attempts left",
                    MAX_ATTEMPTS - attempts,
                    MAX_ATTEMPTS
                );
                continue;
            }
            std::cmp::Ordering::Equal => {
                clear().expect("failed to clear screen");

                println!("You win! {guess} is the correct answer!");
                let response = get_response("Continue? y/n");
                if response == "n" || response == "no" {
                    println!("Thanks for playing!");
                    break;
                } else if response == "y" || response == "yes" {
                    println!("Guess the secret number!");
                    secret = rand::thread_rng().gen_range(1..100); // generate a new secret number
                    continue;
                } else {
                    println!("Invalid response. Please enter a valid response.")
                }
            }
            std::cmp::Ordering::Greater => {
                clear().expect("failed to clear screen");

                println!("{guess} is too large");
                println!(
                    "You have {}/{} attempts left",
                    MAX_ATTEMPTS - attempts,
                    MAX_ATTEMPTS
                );
                continue;
            }
        }
    }
}
fn spanish() {
    println!("Has elegido Español");
    clear().expect("falla al limpiar la pantalla");
    println!("Adivina el número secreto!");

    let mut secret = rand::thread_rng().gen_range(MIN_RANGE..=MAX_RANGE);
    let mut attempts = 0; // counter for the number of attempts made
    loop {
        //println!("{secret}");
        let guess: u32 = get_guess("Adivina: ");
        if guess == 0 {
            clear().expect("falla al limpiar la pantalla");

            println!("Debe ser un entero no cero");
            continue;
        }
        if attempts > MAX_ATTEMPTS - 1 {
            // check if the limit has been reached
            println!("Perdiste. El secreto era {secret}");
            clear().expect("falla al limpiar la pantalla");

            break;
        }
        attempts += 1; // increment the counter

        match guess.cmp(&secret) {
            std::cmp::Ordering::Less => {
                clear().expect("falla al limpiar la pantalla");

                println!("{guess} es demasiado pequeño");
                println!(
                    "Tienes {}/{} intentos restantes",
                    MAX_ATTEMPTS - attempts,
                    MAX_ATTEMPTS
                );
                continue;
            }
            std::cmp::Ordering::Equal => {
                clear().expect("falla al limpiar la pantalla");

                println!("¡Ganaste! {guess} es la respuesta correcta!");
                let response = get_response("Continuar? s/n");
                if response == "n" || response == "no" {
                    println!("Gracias por jugar!");
                    break;
                } else if response == "s" || response == "si" {
                    println!("Adivina el número secreto!");

                    secret = rand::thread_rng().gen_range(1..100); // generate a new secret number
                    continue;
                } else {
                    println!("Respuesta inválida. Por favor ingrese una respuesta válida.")
                }
            }
            std::cmp::Ordering::Greater => {
                clear().expect("falla al limpiar la pantalla");

                println!("{guess} es demasiado grande");
                println!(
                    "Tienes {}/{} intentos restantes",
                    MAX_ATTEMPTS - attempts,
                    MAX_ATTEMPTS
                );
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
    io::stdin().read_line(&mut input).expect("Read Failure");
    input.trim().to_lowercase()
}
