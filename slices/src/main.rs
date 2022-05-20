// fn main() {
//     let mut str = String::from("dog and a cat");
//     let word = first_word(&str);
//     str.clear();

//     println!("{}", word);
// }

// fn first_word(str: &String) -> usize {
//     let bytes = str.as_bytes(); //convert string to an array of bytes

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//     str.len()
// }



// //string slicing
// fn main(){
//     let s = String::from("hello world");
//     let hello = &s[0..3];
//     let world = &s[6..11];

//     println!("{}", hello);

// }


//other slices
// fn main () {
//     let arr = [0, 1, 2, 3, 4];

//     let slice = &arr[0..3];

//     println!("{}", slice[1] +  slice[2]);
// }