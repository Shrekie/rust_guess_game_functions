use std::cmp::Ordering;
use rand::Rng;

mod numbin;

pub fn ask() -> u32 {

    let mut answer = String::new();

    loop {

        if let Some(answer) = 
            numbin::force(&mut answer, "Only numbers please:") {

            break answer;

        }

    }

}

pub fn check(guess: &u32, secret: &u32) -> bool {

    match guess.cmp(&secret) {

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

pub fn serve() -> u32 {

    println!("Guess the random number!"); 
    rand::thread_rng().gen_range(1, 101)

}