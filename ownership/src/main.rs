// use std::io;
// fn main() {
//     println!("Please enter string");
//     let mut a = String::new();
//     io::stdin().read_line(&mut a).expect("Failed to read line");
//     lenght_of_string(a);
//     println!("lenght  is : {} ", a);
// }

// fn lenght_of_string(b: String) {
//     let c = b.trim();
//     println!("The length of {} is {}", c, c.len())
// }

use std::io;
fn main() {
    println!("Enter your string.");
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    //y

    println!(
        "the length of your string:{} is :{}",
        user_input,
        find_length(&user_input)
    );
}

fn find_length(x: &String) -> usize {
    let y = x.trim().len();
    y
}

// let mut a = "hello"; //stack
// let mut b = String::from("10"); //heap
// a.push_str(" Pakistan");
// println!("a is {}", a);
//
// b.push_str(" Pakistan");
// println!("b is {}", b as i32)
