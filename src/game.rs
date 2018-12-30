mod question;

use rand::Rng;

pub struct Guess {

    secret: u32,
    won: bool,

}

impl Guess {

    fn serve() -> u32 {

        println!("Guess the random number!"); 
        rand::thread_rng().gen_range(1, 101)

    }
 
    pub fn new() -> Guess {

        Guess {
            secret: Guess::serve(),
            won: false,
        }

    }

    pub fn cycle(&mut self) {

        loop {

            if self.session() {

                self.restart();

            }

        }

    }

    fn session(&mut self) -> bool {

        while !self.won {
            
            println!("Please input your guess.");
            let guess = question::insert();

            println!("You guessed: {}", guess);
            self.won = question::check(&guess, &self.secret);

        }

        self.won

    }

    fn restart(&mut self) {

        self.secret = Guess::serve();
        self.won = false;

    }

}