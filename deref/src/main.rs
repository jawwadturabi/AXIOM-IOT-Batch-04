fn main() {
    // let mut a: i64 = 0x7ffc00ef5624;
    // let b = &mut a as *mut i64; //
    // let c = &a as *const i64;
    // unsafe {
    //     *b += 3; //value of  a
    // }
    // println!(" a is {:?}", a);
    // let mut address = 0x012345usize;
    // let mut r = address as *const i32;
    // unsafe {
    //     println!("r is {:?}", *r);
    // }
    let mut a = 9;
    unsafe {
        let b = &mut a;
        let c = &a;
        println!("b is {} and c is {}", b, c)
    }
}

// use std::collections::HashMap;
// #[derive(Debug)]

// fn main() {
// let mut a = 4;
// let b = &mut a;
// let c = *b += 3; // b is value of a
// println!("a is {:?}", c);
// println!("a is {}", a);
// println!("b is {}", b)

// let text = "hello world wonderful world";

// let mut map = HashMap::new();

// for word in text.split_whitespace() {
//     let count = map.entry(word).or_insert(0); //&mut value
//     *count += 1;
// }

// println!("{:?}", map);
// }
