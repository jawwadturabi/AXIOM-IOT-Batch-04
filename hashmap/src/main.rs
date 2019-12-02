use std::{collections::HashMap, io};

fn main() {
    // let mut a = String::new();
    // println!("Enter first key ");
    // io::stdin().read_line(&mut a);
    // let a = a.trim();
    // let mut b = String::new();
    // println!("Enter first value ");
    // io::stdin().read_line(&mut b);
    // let b: i32 = b.trim().parse().unwrap();
    // let mut c: HashMap<String, i32> = HashMap::new();
    // let b = vec![
    //     String::from("Team Pakistan"),
    //     String::from("Team Australia"),
    //     String::from("Team England"),
    // ];
    // let c = vec![200, 220, 250];

    let mut a: HashMap<String, i32> = HashMap::new();
    a.insert(String::from("Team Pakistan"), 200);
    a.insert(String::from("Team Australia"), 220);
    a.insert(String::from("Team England"), 250);
    a.entry(String::from("Team Pakistan")).or_insert(320);

    // for (i, j) in &a {
    //     if i == "Team Pakistan" {
    //         println!("{}: {}", i, j);
    //     }
    // }

    println!("a is {:?}", a);
    // let d = String::from("Team Pakistan");
    // let e = a.get(&d);
    // println!("c is {:?}", e);
    // // let mut first_key = String::from("Team Pakistan");
    // let first_value = 200;
    // a.insert(first_key, first_value);
    // c.insert(a.to_string(), b);

    // let team_name = String::from("Blue");
    // let score = scores.get(&team_name);
}
