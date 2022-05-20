//// first way to do it
// fn main() {
//     let x = 40;
//     let y = 23;

//     println!("The area of the rectangle is {}", calculate_rectangle_area(x, y));
// }

// fn calculate_rectangle_area(length: i32, width: i32) -> i32 {
//     length * width
// }


// //now written with tuples
// fn main() {
//     let rectangle1 = (20, 20);  

//     println!("The area is {}", calculate_rectangle_area(rectangle1));

// }

// fn calculate_rectangle_area(dimensions: (i32, i32)) -> i32 {
//     dimensions.0 * dimensions.1
// }













//now even better with structs

#[derive(Debug)] //allows for printing the struct below
struct Rectangle {
    width: i32,
    height: i32,
}

//defining a *method* to calculate area for the struct instead of a function

impl Rectangle {
    fn area(&self) -> i32 {
        self.width * self.height
    }

    fn can_contain(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    //associated function
    fn square(size: i32) -> Rectangle {
        Rectangle {width: size, height: size}
    }
}


//function NOT a method
// fn calculate_area(rectangle: &Rectangle) -> i32 {
//     rectangle.width * rectangle.height
// }

fn main() {

    let rect1 = Rectangle {width: 30, height: 20};
    let rect2 = Rectangle {width: 20, height: 14};
    let rect3 = Rectangle {width: 45, height: 30};

    //instantiating a square
    let sq = Rectangle::square(5);

    // println!("Rectangle one is: {:?}", rect1);

    // println!("Rectangle one is: {:#?}", rect1);

    // println!("The area of rect1 is  {}", calculate_area(&rect1));

    // println!("Using a method to calculate the area: {}", rect1.area());

    // if (rect1.can_contain(&rect2)) {
    //     println!("Rect1 can contain rect2");
    // }


    // println!("Can rect1 contain rect3? {}", rect1.can_contain(&rect3));

    println!("The area of a the square {}", sq.area());
    
}
