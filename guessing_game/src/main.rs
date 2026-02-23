use std::cmp::Ordering;
use std::io;
use std::num::{IntErrorKind, ParseIntError};

fn main() {
    // 範囲式は「開始..=終了」の形で表す、下限値と上限値ともに含む
    let secret_number = rand::random_range(1..=100);

    println!("Guess the number");

    loop {
        println!("Please input your guess.");

        // mutは可変な変数を表す(mutable)、デフォルトでは不変変数(immutable)
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // Rustでは前の変数を新しい変数でshadowすることが可能
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                match err.kind() {
                    IntErrorKind::InvalidDigit => println!("Please input a positive number."),
                    _ => println!("Error: {err}")
                }
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
    println!("The secret number is: {secret_number}");
}
