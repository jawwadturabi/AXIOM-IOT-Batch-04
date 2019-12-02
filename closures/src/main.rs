// use std::thread;
// use std::time::Duration;
// fn generate_workout(intensity: u32, random_number: u32) {
//     let calculate = |a| {
//         println!("calculating slowly...");
//         thread::sleep(Duration::from_secs(2));
//         a
//     };
//     if intensity < 25 {
//         println!("Today, do {} pushups!", calculate(intensity));

//         println!("Next, do {} situps!", calculate(intensity));
//     } else {
//         if random_number == 3 {
//             println!("Take a break today! Remember to stay hydrated!");
//         } else {
//             println!("Today, run for {} minutes!", calculate(intensity));
//         }
//     }
// }
fn main() {
    // let a = 5;
    // let my_closure = |mut b| -> i32 { b = a };

    // let c = 4;
    // println!("a is {} , b is {:?}", a, my_closure(c));
    // let x = 4;
    // let value = |z: i32| -> bool { z == x };
    // assert!(value(4));

    // let equal_to_x = |z| z == x;

    // let y = 4;

    // assert!(equal_to_x(y));

    let x = vec![1, 2, 3];

    let equal_to_x = move |z| {
        println!("z is : {:?}", z);
        z == x
    };
    // println!("z is : {:?}", z);

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}

// fn workout_calculation(intensity: u32) -> u32

// fn main() {
// let simulated_user_specified_value = 20; //take intensity from user
// let simulated_random_number = 3;

// let workout = |intensity| {
//     println!("today do some workout");
//     intensity
// };
// println!("workout is {}", workout(String::from("hello")));
// println!("workout is {}", workout(22));

// generate_workout(simulated_user_specified_value, simulated_random_number);
// }
