use std::io;
use std::cmp::Ordering;
use rand::Rng;
use std::io::Write;

fn main() {
    
    loop {
        print!("Input a number: ");
        io::stdout().flush().unwrap();

        //mut stands for mutable //::indicates that new is an associated function
        let mut guess = String::new();
        
        let secret_number = rand::thread_rng().gen_range(1, 6);

        //the &mut indicates that this argument is a reference
        io::stdin().read_line(&mut guess)
            .expect("Failed to read input");
        
        //convert the String type the program reads as input to an integer to be used for comparision below, this is a shadow of the guess variable above    

        //the colon after guess specifies, or annotates the variables type, which is u32: an unsigned 32 bit integer

        //parse returns a Result type and Result is an enum that has the variants Ok or Err
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small..."),
            Ordering::Greater => println!("Too big..."),
            Ordering::Equal => {
                println!("You guessed it!");
                break;
            }
            
        
        }
  
        println!("The secret number was  {} \n", secret_number);
        
        let mut cont = String::new();
        println!("Press enter to continue");
        io::stdin().read_line(&mut cont)
            .expect("Error");

        clearscreen::clear().unwrap();
    }
}




