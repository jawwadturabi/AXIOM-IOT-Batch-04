fn main() {
    let a = Some(4);
    let b = "hello";
    match b {
        "hello" => println!("hello triggered"),
        _ => (),
    }
    converter(a);
}
fn converter(curr: Option<i32>) {
    match curr {
        Some(5) => println!("entered currency is pkr"),
        Some(4) => println!("entered currency is $"),
        _ => println!("invalid"),
    }
}
