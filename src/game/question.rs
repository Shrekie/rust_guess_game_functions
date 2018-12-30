use std::io;
use std::cmp::Ordering;

pub fn insert() -> u32 {

    let mut guess = String::new();
    
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse() {

        Ok(num) => num,
        Err(_) => {
            println!("Only numbers please");
            insert()
        }

    };

    return guess;

}

pub fn check(guess: &u32, secret_number: &u32) -> bool {

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