// fn main() {
//     function_one(14, -72);
// }

// //in function signatures, you MUST declare the type of each parameter
// fn function_one(y: i32, z: i8) {
//     println!("The value of y is {}", y);
//     println!("The value of z is {}", z);
// }

// fn main() {
//     let x = 5;

//     //expressions do not include semi colons
//     let y = {
//         let x = 3;
//         x + 1
//     };

//     println!("The value of y {}", y);

// }


// //returning values from functions
// fn main(){
//     let q = four();

//     println!("{}", q);
// }

// fn four() -> i32 {
//     4
// }


// //another function example
// fn main() {
//     let x = times_ten(10);
//     println!("{}", x);
// }

// //adding a semicolon fails because it turns the line from an epxression into a statement
// fn times_ten(x: i32) -> i32 {
//     x * 10
// }


// //if Expressions
// fn main() {
//     let conditional = false;

//     let x = if conditional{
//         5
//     } else {
//         6
//     };

//     println!("X: {}", x);
// }
