// fn string_length(a: String) -> usize {
//     a.len()
// }

// fn main() {
// let x = "hello";
// let y = String::from("hello");
// let z = "hello".to_string();

// println!("x is : {}", x);
// println!("y is : {}", y);
// println!("z is : {}", z)
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
// }
