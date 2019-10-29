// input/output library
use std::io;
// comparison library
use std::cmp::Ordering;
// randomizer library
use rand::Rng;

fn main() {
    // print line
    println!("Guess the number!");
    // thread_rng() - thread range uses a single thread when empty
    let secret_number = rand::thread_rng().gen_range(1, 11);

    // for debug use
    // println!("The secret number is: {}", secret_number);

    // loop over to allow infinite guesses until match is made
    loop {
        println!("Please input your guess.");

        // mut = mutable variable
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // changed the variable type of guess to a 32bit integer
        // this is possible due the mutable variable type assigned earlier
        let guess: u32 = match guess.trim().parse() {

            // validate user inpou, only allow int
            Ok(num) => num,
            Err(_) => {
                // notif user of error
                println!("Enter a number, like 1");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        // check if there is a match by comparing guess with secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Correct!");
                // exit if guessed correctly
                break;
            }
        }
    }
}
