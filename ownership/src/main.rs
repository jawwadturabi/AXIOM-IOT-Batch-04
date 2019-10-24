fn main() {
    let a = String::from("hello"); //hello
    let c = lenght_of_string(a); //(length,string)
    println!("lenght  is : {} b is {}", c.0, c.1);
}

fn lenght_of_string(b: String) -> (usize, String) {
    (b.len(), b)
}

// let mut a = "hello"; //stack
// let mut b = String::from("10"); //heap
// a.push_str(" Pakistan");
// println!("a is {}", a);
//
// b.push_str(" Pakistan");
// println!("b is {}", b as i32)
