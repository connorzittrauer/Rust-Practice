// fn main() {
//     // let mut name = String::from("bob");
//     // {
//     //     let reference1 = &mut name;
//     // } //multiple reference allowed because reference1 goes out of scope at this project, this is compilable

//     // let reference2 = &mut name;





// }

// //mutable vs immutable references
// fn main {
//     let mut name = String::from("bob");
//     let r1 = &mut name; //the push_str works because of the reference to &mut
//     //let r1 = &name; //this fails because the the &name cannot be borrowed as mutable

//     r1.push_str(" smith");
    
    
//     println!("{}", name);



// //dangling reference 
// fn main() {
//     let nothing = dangling_ref();
// }

// fn dangling_ref() -> &String {
//     let str = String::from("asdf");
//     &str
// }//str is deallocated here, thus we are returning an empty string
