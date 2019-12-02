use std::io;
fn main() {
    // println!("Hello, world!");
    // let mut b = "Pakistan";
    // b.push_str("Zindabad");
    // a.push_str("hello");

    // let mut a = String::from("hello");
    // println!("a is {}", a);
    // a += &String::from(", World");
    // println!("a is {}", a);

    // println!("b is {}", b)
    // let s1 = String::from("Hello, ");
    // let s2 = String::from("world!");
    // let s3 = s1 + &s2;
    // println!("s3 is {}", s3);
    // println!("s2 is {}", s2);
    // println!("s1 is {}", s1);
    //     let s1 = String::from("tic");
    //     let s2 = String::from("tac");
    //     let s3 = String::from("toe");

    //     let s = s1 + "-" + &s2 + "-" + &s3;
    //     let c = format!("{}-{}", s2, s3);
    //     println!("c is : {:?}", c)

    // let a = String::from("Pakistan");
    // println!("first letter is {}", a[0])
    // let len1 = String::from("Hola").len();
    // let len2 = String::from("Здравствуйте").len();
    // let a = "hello";
    // for i in a.bytes() {
    //     println!("i is {}", i)
    // }
    // let mut a = String::new();
    // println!("Enter first string ");
    // io::stdin().read_line(&mut a);
    // let a = a.trim();
    // let mut b = String::new();
    // println!("Enter second string ");
    // io::stdin().read_line(&mut b);
    // let b = b.trim();
    // let c = a.to_owned() + " " + b;
    // println!("c is {}", c)
    let mut a = String::from("hell");
    println!("a is {} ", a);
    a.push('o');
    println!("a is {} ", a)
}
// fn add(self,s:&str)>String

// fn myfnc(a: &String, b: String) {
//     println!("a is {} b is {}", a, b)
// }
