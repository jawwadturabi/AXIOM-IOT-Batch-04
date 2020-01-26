use std::f32;
// fn main() {
//     let r;

//     let x = 5;
//     r = &x;

//     println!("r: {}", r);
// }

// fn main() {
//     let string1 = String::from("abcd");
//     let result;
//     let string2 = String::from("xyz");
//     result = longest(string1.as_str(), string2.as_str());
//     println!("The longest string is {}", result);
// }

// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }
// fn main() {
#[derive(Debug)]
struct seasons<'a> {
    Summer: &'a str,
    Winter: &'a str,
    Autumn: String,
    Spring: String,
}
fn main() {
    let seas1 = seasons {
        Summer: "No",
        Winter: "Yes",
        Autumn: String::from("No"),
        Spring: String::from("No"),
    };
    let seas2 = seasons {
        Summer: seas1.Summer,
        Winter: seas1.Winter,
        Autumn: String::from("No"),
        Spring: String::from("No"),
    };
    println!("seas1 is {:#?}", seas1);
    // let reference_to_nothing = dangle();
}

// fn dangle() -> &str {
//     let s = "hello";

//     s
// }
// let a = b'@';
// println!("a is {:b}", a);
// let a: i32 = 2; // Bit presentation 10
// let b: i32 = 3; // Bit presentation 11

// println!("addition  is {}", a + b);

// let mut result: i32;

// result = a & b; // and operator bitwise multiplication
// println!("(a & b) => {} ", result);

// result = a | b; //or operator bitwise addition
// println!("(a | b) => {} ", result);

// result = a ^ b; //xor
// println!("(a ^ b) => {} ", result);

// result = !b; //not
// println!("(!b) => {} ", result);

// result = a << b; //left shift
// println!("(a << b) => {}", result);

// result = a >> b; //right shift
// println!("(a >> b) => {}", result);
// }
