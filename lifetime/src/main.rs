// fn main() {
//     let r;

//     let x = 5;
//     r = &x;

//     println!("r: {}", r);
// }

fn main() {
    let string1 = String::from("abcd");
    let result;
    let string2 = String::from("xyz");
    result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
