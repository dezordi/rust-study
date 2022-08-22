use std::io;                                  //here from the standart library "std" we import "::" the "io" library, the "io" librari allows we work with input
use rand::Rng;                                //output provided by the user.
use std::cmp::Ordering;
fn main() {

    // Getting a number from user
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();           // let is the statement to createa a variable, by default all variables in rust are immutables, so we need to declared "mut"
                                                // to a mutable variable, the String::new() statemen is used to create new empty string.

        io::stdin()                              // Here, we declared the statement io::stdin() to allow us to handle with user input.
            .read_line(&mut guess)               // the read_line method allows to receive information as user input, and store it in the mutable guess variable.
            .expect("Failed to read line");
        
        let guess: u32 = match guess.trim().parse() { // The trimm method will remove any space. The parsed method convert the string type. 
            Ok(num) => num,
            Err(_) => continue, //The underscore, _, is a catchall value; in this example, we’re saying we want to match all Err values, no matter what information they have inside them
        }; 

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break}
        }
    }
}