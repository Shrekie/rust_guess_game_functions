mod question;

pub struct Guess {

    secret: u32,
    won: bool,

}

impl Guess {
 
    pub fn new() -> Guess {

        Guess {
            secret: question::serve(),
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

        loop {
            
            println!("Please input your guess:");
            let guess = question::ask();

            println!("You guessed: {}", guess);
            if let true = question::check(&guess, &self.secret) {

                break true;
                
            }

        }

    }

    fn restart(&mut self) {

        self.secret = question::serve();
        self.won = false;

    }

}