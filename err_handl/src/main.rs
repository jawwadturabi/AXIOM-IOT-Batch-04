// use std::fs;
// use std::io;
// use std::net::IpAddr;
// use std::io::Read;
// use std::io::ErrorKind;
// fn main() {
// let f = File::open("hello.txt");
// let f = match f {
//     Ok(file) => file,
//     Err(error) => match error.kind() {
//         ErrorKind::NotFound => match File::create("index.txt") {
//             Ok(fc) => {
//                 println!("file created successfully");
//                 fc
//             }
//             Err(e) => panic!("problem creating file: {:#?} ", e),
//         },
//         other_error => panic!("Problem opening the file: {:#?}", other_error),
//     },
// };
// let f = File::open("hello.txt")?;
// match read_username_from_file() {
//     Ok(a) => println!("content are :{:#?} ", a),
//     Err(e) => panic!("error is :{:#?}", e),
// }
// }
// fn read_username_from_file() -> Result<String, io::Error> {
// fs::read_to_string("hello.txt")

// let mut s = String::new();
// let f = File::open("hello.txt")?.read_to_string(&mut s)?;
// Ok(s)

// let mut f = match f {
//     Ok(file) => file,
//     Err(e) => return Err(e),
// };
//  match f.read_to_string(&mut s) {
//     Ok(_) => Ok(s),
//     Err(e) => Err(e),
// }
// }

// let a = [23, 45, 67];
// let i = 10;
// println!("a is {}", a[i]);
struct Array {
    index: i32,
}

impl Array {
    // fn new(value: i32) -> Array {
    //     if value >= 0 && value < 3 {
    //         println!("The index must be between 0 and 3, got {}.", value);
    //     }

    //     Array { index: value }
    // }

    fn index(&self) -> i32 {
        self.index
    }
}
fn main() {
    let a = [3, 4, 5];
    let b = 3;
    let mut my_index = Array { index: b };
    println!("{:#?}", my_index.index());
    // loop {
    //     let mut guess = String::new();

    //     io::stdin()
    //         .read_line(&mut guess)
    //         .expect("Failed to read line");
    //     // --snip--

    //     let guess:i32 = match guess.trim().parse() {
    //         Ok(num) => Ok(num),
    //         Err(_) => continue,
    //     };

    //     if guess < 1 || guess > 100 {
    //         println!("The secret number will be between 1 and 100.");
    //         continue;
    //     }

    // match guess.cmp(&secret_number) {
    // --snip--
    // }
    // let guess: IpAddr = guess.trim().parse().unwrap();
    // println!("guess is {}", guess)
}
