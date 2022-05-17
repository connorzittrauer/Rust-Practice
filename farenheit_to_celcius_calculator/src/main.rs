// //reddit improvement
use std::io::{self, Write as _};


fn main() {
    //mutable  variable, set by the user
    let mut fahrenheit = String::new();

    loop {
        fahrenheit.clear();

        //get input from user
        print!("Input the degrees in fahrenheit: ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut fahrenheit)
            .expect("Erroneous input");

        let fahrenheit = fahrenheit.trim();
        if fahrenheit.eq_ignore_ascii_case("q") {
            break;
        }    

        let fahrenheit: f64 = match fahrenheit.parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let celsius = (fahrenheit - 32.00) * 0.5556;

        println! (
            "{} degrees fahrenheit is {} degrees celsius \n",
            fahrenheit, celsius
        );


    }

}
