fn main() {
    
    //entire instance of Struct must be mutable, can't makr individual fields as mutable
    let mut car1 = Car {
        make: String::from("Ford"),
        model: String::from("F 150"),
        year: 2009,
        operational: true,
    };    
    
    //reusing values from the first instance of car
    let car2 = Car {
        make: String::from("Toyota"),
        model: String::from("Tundra"),
        // year: car1.year,
        // operational: car1.operational,
        
        ..car1    //implicit syntax fills the  rest of the  fields
    };


    //using dot notation to get the value
    println!("{}", car1.make);


    //mutating the field
    car1.make = String::from("Dodge");

    println!("Car 1: {}, Car 2 {}", car1.make, car2.make);


    let entries = Catalogs(123, 232, 001);
    let names = Names(String::from("bob"));


}


struct Car {
    make: String,
    model: String,
    year: i32,
    operational: bool,
}




//tuple structs
struct Catalogs(u32, u32, u32);
struct Names(String);