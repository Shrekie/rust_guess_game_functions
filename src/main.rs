use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn generate_random() -> u32 {

    rand::thread_rng().gen_range(1, 101)

}

fn input_guess() -> u32 {

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
    .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => input_guess(),
    };

    guess

}

fn evaluate_guess(guess: &u32, secret_number: &u32) {

    match guess.cmp(&secret_number) {

        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            main();
        }

    };

}

fn main() {

    println!("Guess the number!");

    let secret_number = generate_random();

    loop {
        
        println!("Please input your guess.");

        let guess = input_guess();

        println!("You guessed: {}", guess);

        evaluate_guess(&guess, &secret_number);

    }

}