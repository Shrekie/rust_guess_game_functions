use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn generate_random() -> u32 {

    println!("Guess the random number!"); 
    rand::thread_rng().gen_range(1, 101)

}

fn input_guess() -> u32 {

    let mut guess = String::new();
    
    io::stdin().read_line(&mut guess)
    .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse() {

        Ok(num) => num,
        Err(_) => {
            println!("Only numbers please");
            input_guess()
        }

    };

    guess

}

fn evaluate_guess(guess: &u32, secret_number: &u32) -> bool {

    match guess.cmp(&secret_number) {

        Ordering::Less => {
            println!("Too small!");
            false
        },
        Ordering::Greater => { 
            println!("Too big!");
            false
        },
        Ordering::Equal => {
            println!("You win!");
            true
        }

    }

}

fn guess_session(secret_number: &u32) -> bool {

    let mut won_guess = false;

    while !won_guess {

        println!("Please input your guess");
        let guess = input_guess();

        println!("You guessed: {}", guess);
        won_guess = evaluate_guess(&guess, &secret_number)

    }

    won_guess

}

fn stage_guess() {

    let mut secret_number = generate_random();

    if guess_session(&secret_number) {

        secret_number = generate_random();
        guess_session(&secret_number);

    }

}

fn main() {

    stage_guess()

}