#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

fn main() {
    let rect1 = Rectangle {
        height: 3,
        width: 5,
    };
    println!("area of rectangle is :{}", area_of_rectangle(rect1));

    fn area_of_rectangle(d: Rectangle) -> u32 {
        d.height * d.width
    }
}

// let dimensions = (3, 5);
// println!("area of rectangle is :{}", area_of_rectangle(dimensions));

// fn area_of_rectangle(d: (i32, i32)) -> i32 {
//     d.0 * d.1
// }
// #[derive(Debug)]
// struct User {
//     name: String,
//     email: String,
//     age: i32,
//     city: String,
// }
// struct units{

// }

// fn main() {
//     let mut user1 = User {
//         name: "Ali".to_string(),
//         email: "ali@abc.com".to_string(),
//         age: 34,
//         city: "karachi".to_string(),
//     };
//     // println!("{}", user1.name);
//     // println!("{}", user1.email);
//     // println!("{}", user1.age);
//     // println!("{}", user1.city);

//     for i in user1.iter()
//     {
//         println!("{}", i)
//     }
// }

// let user1 = make_instances("Ali".to_string(), "ali@abc.com".to_string(), 23);
// let user2 = make_instances("Ahmed".to_string(), "ahmed@abc.com".to_string(), user1.age);
// let user3 = make_instances("Aslam".to_string(), "aslam@abc.com".to_string(), 27);
// let user4 = make_instances("Fareed".to_string(), "fareed@abc.com".to_string(), 28);

// println!("{:#?}", user1);
// println!("{:#?}", user2);
// println!("{:#?}", user3);
// println!("{:#?}", user4);

// fn make_instances(name: String, email: String, age: i32) -> User {
//     User {
//         email,
//         name,
//         age,
//         city: "karachi ".to_string(),
//     }
// }

// let user1: (String, String, i32, f32) = (18, "ali@abc.com", "ali", 5.5);
// let (name, email, age, height) = user1;

// let user2 = ("ahmed", "ahmed@abc.com", 20, 5.7);
// let (name, email, age, height) = user2;

// let user3 = ("aslam", "aslam@abc.com", 22, 5.6);
// let (name, email, age, height) = user3;

// let user4 = ("fareed", "fareed@abc.com", 17, 5.4);
// let (name, email, age, height) = user4;

// println!("{}", user2)
