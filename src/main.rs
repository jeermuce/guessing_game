use clearscreen::clear;
use rand::Rng;
use std::io::{self, stdout, Write};

const MAX_ATTEMPTS: u32 = 10;
const MIN_RANGE: u32 = 1;
const MAX_RANGE: u32 = 100;
macro_rules! game {
    ($lang:expr) => {
        match $lang {
            0 => {
                println!("Thanks for playing!");
                break;
            }
            1 => english(),
            2 => spanish(),
            3 => german(),
            4 => japanese(),
            5 => korean(),
            6 => latin(),
            _ => clear().expect("failed to clear screen"),
        }
    };
}
fn main() {
    loop {
        let language: u16 = get_response(
            "
Quit/Salir/Beenden/終了/종료/Exit:____0: Quit
Choose a language:____________________1: English
Elija un idioma:______________________2: Español     
Wählen Sie eine Sprache aus:__________3: Deutsch
言語を選択してください:_______________4: 日本語
언어를 선택하십시오:__________________5: 한국어
Linguam eligite:______________________6: Lingua latina

",
        )
        .parse()
        .expect("Must be a number");
        game!(language);
    }
}

fn english() {
    println!("You have chosen English");
    clear().expect("failed to clear screen");
    let mut secret = rand::thread_rng().gen_range(MIN_RANGE..=MAX_RANGE);
    let mut attempts = 0;
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
            println!("You lose. The secret was {secret}");
            clear().expect("failed to clear screen");
            break;
        }
        attempts += 1;
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
                    clear().expect("continuation failure");
                    println!("Guess the secret number!");
                    secret = rand::thread_rng().gen_range(1..100);
                    attempts = 0;
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
    let mut secret = rand::thread_rng().gen_range(MIN_RANGE..=MAX_RANGE);
    let mut attempts = 0;
    println!("Adivina el número secreto!");
    loop {
        //println!("{secret}");
        let guess: u32 = get_guess("Adivina: ");
        if guess == 0 {
            clear().expect("falla al limpiar la pantalla");
            println!("Debe ser un entero no cero");
            continue;
        }
        if attempts > MAX_ATTEMPTS - 1 {
            println!("Perdiste. El secreto era {secret}");
            clear().expect("falla al limpiar la pantalla");
            break;
        }
        attempts += 1;

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
                    clear().expect("falla al continuar");
                    println!("Adivina el número secreto!");
                    secret = rand::thread_rng().gen_range(1..100);
                    attempts = 0;
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
fn german() {
    println!("Sie haben Deutsch gewählt");
    clear().expect("Fehler beim Löschen des Bildschirms");
    let mut secret = rand::thread_rng().gen_range(MIN_RANGE..=MAX_RANGE);
    let mut attempts = 0;
    println!("Errate die geheime Zahl!");
    loop {
        //println!("{secret}");
        let guess: u32 = get_guess("Raten: ");
        if guess == 0 {
            clear().expect("Fehler beim Löschen des Bildschirms");
            println!("muss eine ganze Zahl ungleich Null sein");
            continue;
        }
        if attempts > MAX_ATTEMPTS - 1 {
            println!("Du verlierst. Die geheime Zahl war {secret}");
            clear().expect("Fehler beim Löschen des Bildschirms");
            break;
        }
        attempts += 1;

        match guess.cmp(&secret) {
            std::cmp::Ordering::Less => {
                clear().expect("Fehler beim Löschen des Bildschirms");
                println!("{guess} ist zu klein");
                println!(
                    "Du hast noch {}/{} Versuche übrig",
                    MAX_ATTEMPTS - attempts,
                    MAX_ATTEMPTS
                );
                continue;
            }
            std::cmp::Ordering::Equal => {
                clear().expect("Fehler beim Löschen des Bildschirms");
                println!("Du gewinnst! {guess} ist die richtige Antwort!");
                let response = get_response("Weitermachen? j/n");
                if response == "n" || response == "nein" {
                    println!("Danke fürs Spielen!");
                    break;
                } else if response == "j" || response == "ja" {
                    clear().expect("Fehler beim Weitermachen");
                    println!("Errate die geheime Zahl!");
                    secret = rand::thread_rng().gen_range(1..100);
                    attempts = 0;
                    continue;
                } else {
                    println!("Ungültige Antwort. Bitte geben Sie eine gültige Antwort ein.")
                }
            }
            std::cmp::Ordering::Greater => {
                clear().expect("Fehler beim Löschen des Bildschirms");
                println!("{guess} ist zu groß");
                println!(
                    "Du hast noch {}/{} Versuche übrig",
                    MAX_ATTEMPTS - attempts,
                    MAX_ATTEMPTS
                );
                continue;
            }
        }
    }
}
fn japanese() {
    println!("日本語を選択しました");
    clear().expect("画面クリアに失敗しました");
    let mut secret = rand::thread_rng().gen_range(MIN_RANGE..=MAX_RANGE);
    let mut attempts = 0;
    println!("秘密の数字を当ててください！");
    loop {
        //println!("{secret}");
        let guess: u32 = get_guess("予想: ");
        if guess == 0 {
            clear().expect("画面クリアに失敗しました");
            println!("0以外の整数を入力してください");
            continue;
        }
        if attempts > MAX_ATTEMPTS - 1 {
            println!("あなたの負けです。秘密の数字は {} でした", secret);
            clear().expect("画面クリアに失敗しました");
            break;
        }
        attempts += 1;

        match guess.cmp(&secret) {
            std::cmp::Ordering::Less => {
                clear().expect("画面クリアに失敗しました");
                println!("{guess} は小さすぎます");
                println!(
                    "残りの試行回数は {}/{} 回です",
                    MAX_ATTEMPTS - attempts,
                    MAX_ATTEMPTS
                );
                continue;
            }
            std::cmp::Ordering::Equal => {
                clear().expect("画面クリアに失敗しました");
                println!("当たり！ {guess} が正解です！");
                let response = get_response("続けますか？ y/n");
                if response == "n" || response == "いいえ" {
                    println!("お疲れ様でした！");
                    break;
                } else if response == "y" || response == "はい" {
                    clear().expect("続行に失敗しました");
                    println!("秘密の数字を当ててください！");
                    secret = rand::thread_rng().gen_range(1..100);
                    attempts = 0;
                    continue;
                } else {
                    println!("無効な入力です。有効な入力をしてください。")
                }
            }
            std::cmp::Ordering::Greater => {
                clear().expect("画面クリアに失敗しました");
                println!("{guess} は大きすぎます");
                println!(
                    "残りの試行回数は {}/{} 回です",
                    MAX_ATTEMPTS - attempts,
                    MAX_ATTEMPTS
                );
                continue;
            }
        }
    }
}
fn korean() {
    println!("한국어를 선택하셨습니다");
    clear().expect("화면을 지우는데 실패하였습니다.");
    let mut secret = rand::thread_rng().gen_range(MIN_RANGE..=MAX_RANGE);
    let mut attempts = 0;
    println!("비밀 숫자를 맞춰보세요!");
    loop {
        //println!("{secret}");
        let guess: u32 = get_guess("추측: ");
        if guess == 0 {
            clear().expect("화면을 지우는데 실패하였습니다.");
            println!("0이 아닌 정수를 입력해주세요.");
            continue;
        }
        if attempts > MAX_ATTEMPTS - 1 {
            println!("게임 오버. 비밀 숫자는 {secret}였습니다.");
            clear().expect("화면을 지우는데 실패하였습니다.");
            break;
        }
        attempts += 1;

        match guess.cmp(&secret) {
            std::cmp::Ordering::Less => {
                clear().expect("화면을 지우는데 실패하였습니다.");
                println!("{guess}은(는) 너무 작습니다.");
                println!("남은 기회: {}/{}", MAX_ATTEMPTS - attempts, MAX_ATTEMPTS);
                continue;
            }
            std::cmp::Ordering::Equal => {
                clear().expect("화면을 지우는데 실패하였습니다.");
                println!("정답입니다! {guess}를 맞추셨습니다!");
                let response = get_response("계속하시겠습니까? y/n");
                if response == "n" || response == "no" {
                    println!("게임 종료!");
                    break;
                } else if response == "y" || response == "yes" {
                    clear().expect("화면을 지우는데 실패하였습니다.");
                    println!("비밀 숫자를 맞춰보세요!");
                    secret = rand::thread_rng().gen_range(1..100);
                    attempts = 0;
                    continue;
                } else {
                    println!("잘못된 입력입니다. 다시 입력해주세요.");
                }
            }
            std::cmp::Ordering::Greater => {
                clear().expect("화면을 지우는데 실패하였습니다.");
                println!("{guess}은(는) 너무 큽니다.");
                println!("남은 기회: {}/{}", MAX_ATTEMPTS - attempts, MAX_ATTEMPTS);
                continue;
            }
        }
    }
}
fn latin() {
    println!("Linguam elegisti Latinam");
    clear().expect("fallit deletio ecrani");
    let mut secret = rand::thread_rng().gen_range(MIN_RANGE..=MAX_RANGE);
    let mut attempts = 0;
    println!("Conjectura numerum secretum!");
    loop {
        //println!("{secret}");
        let guess: u32 = get_guess("Conjectura: ");
        if guess == 0 {
            clear().expect("fallit deletio ecrani");
            println!("Debet esse numerus integer non nullus");
            continue;
        }
        if attempts > MAX_ATTEMPTS - 1 {
            println!("Amisisti. Secretum erat {secret}");
            clear().expect("fallit deletio ecrani");
            break;
        }
        attempts += 1;

        match guess.cmp(&secret) {
            std::cmp::Ordering::Less => {
                clear().expect("fallit deletio ecrani");
                println!("{guess} est parvus nimis");
                println!(
                    "Habes {}/{} conationes reliquas",
                    MAX_ATTEMPTS - attempts,
                    MAX_ATTEMPTS
                );
                continue;
            }
            std::cmp::Ordering::Equal => {
                clear().expect("fallit deletio ecrani");
                println!("Vicisti! {guess} est responsio vera!");
                let response = get_response("Perge? y/n");
                if response == "n" || response == "no" {
                    println!("Gratias tibi ago pro ludendo!");
                    break;
                } else if response == "y" || response == "yes" {
                    clear().expect("fallit resumptio");
                    println!("Conjectura numerum secretum!");
                    secret = rand::thread_rng().gen_range(1..100);
                    attempts = 0;
                    continue;
                } else {
                    println!("Responsio invalida. Placere valide responsionem inserere.")
                }
            }
            std::cmp::Ordering::Greater => {
                clear().expect("fallit deletio ecrani");
                println!("{guess} est magnus nimis");
                println!(
                    "Habes {}/{} conationes reliquas",
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
