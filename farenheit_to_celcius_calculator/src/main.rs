use std::io;
use std::io::Write;

fn main() {

        loop {
            //get farenheit input from user
            print!("Input the degrees in farenheit (numbers only): ");
            io::stdout().flush().unwrap();
    

            //mutable farenheit variable set by user
            let mut farenheit = String::new();
        
            io::stdin().read_line(&mut farenheit)
                .expect("Failed to read input");    

            let farenheit: i32 = match farenheit.trim().parse(){
                Ok(num) => num,
                Err(_) => continue,

            };

          

            let farenheit_float = farenheit as f64;
         
            let celcius: f64 = (farenheit_float - 32.00) * 0.5556;
            
            println!("{} degrees farenheit is {} degrees celcius \n", farenheit, celcius);
            
        }


}
