use std::io;
use std::cmp::Ordering;
use rand::Rng;

struct Question;

impl Question {

    fn insert() -> u32 {

        let mut guess = String::new();
        
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {

            Ok(num) => num,
            Err(_) => {
                println!("Only numbers please");
                Question::insert()
            }

        };

        guess

    }

    fn check(guess: &u32, secret_number: &u32) -> bool {

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

}

struct Guess {

    secret: u32,
    won: bool,

}

impl Guess {

    fn serve() -> u32 {

        println!("Guess the random number!"); 
        rand::thread_rng().gen_range(1, 101)

    }

    fn new() -> Guess {

        Guess {
            secret: Guess::serve(),
            won: false,
        }

    }

    fn cycle(&mut self) {

        loop {

            if self.session() {

                self.restart();

            }

        }

    }

    fn session(&mut self) -> bool {

        while !self.won {
            
            println!("Please input your guess.");
            let guess = Question::insert();

            println!("You guessed: {}", guess);
            self.won = Question::check(&guess, &self.secret);

        }

        self.won

    }

    fn restart(&mut self) {

        self.secret = Guess::serve();
        self.won = false;

    }

}

fn main() {

    let mut guess = Guess::new();

    guess.cycle();

}