// fn main() {
//     let ipv4 = IPType::V4(String::from("8.8.8.8"));
//     let ipv6 = IPType::V6(String::from("::1"));
    
//     let hexa = ColorType::Hexadecimal(String::from("#FFFFFF"));
//     let rgba = ColorType::RGBA(1, 1 ,1, 1);

//     //let ipv6 = IPType::V6;



//     // let home_ip = IP_ADDRESS {
//     //     ip_type: IPType::V4,
//     //     address: String::from("8.8.8.8"),
//     // };

//     // let loop_back = IP_ADDRESS {
//     //     ip_type: IPType::V6,
//     //     address: String::from("::1"),
//     // };

// }

// enum IPType {
//     V4(String),
//     V6(String),
// }

// enum ColorType {
//     Hexadecimal(String),
//     RGBA(i32, i32, i32, i32),

// }

// fn route(ip_type: IPType) { }






// fn main() {
//     let message = Message::Write(String::from("a message"));
//     message.call();
// }


// enum Message {
//     Quit,
//     Move { x: i32, y: i32 },
//     Write (String),
//     ChangeColor (i32, i32, i32),
// }

// impl Message {
//     fn call(&self) {
//         println!("Calling!");
//     }
// }



////Option<T> Enum example 
// fn main() {
//     let num1 = Some(5);
//     let str1 = Some("a string!");

//     let absent_number: Option<i32> = None;
// }


// //The match control flow operator
// fn main() {
//     let penny = Coin::Penny;
//     let quarter1 = Coin::Quarter(State::Louisiana);

//     let val = value_in_cents(penny);

//     value_in_cents(quarter1);
//     println!("{}", val);
// }

// #[derive(Debug)]
// enum State {
//     Texas,
//     Louisiana,
// }

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(State),
// }


// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("State quarter from {:?}!", state);
//             25
//         },
//     }
// }

fn main() {

    let some_u8_value = 89;

    match some_u8_value {
        1 => println!("One!"),
        4 => println!("Four!"),
        6 => println!("Six!"),
        _ => (),
    }
}



// let some_u8_value = 0u8;
// match some_u8_value {
// 1 => println!("one"),
// 3 => println!("three"),
// 5 => println!("five"),
// 7 => println!("seven"),
// _ => (),
// }