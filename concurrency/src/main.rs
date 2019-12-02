use std::thread;
use std::time::Duration;
fn main() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
        drop(v);
        println!("Here's a vector: {:?}", v);
    });

    thread::sleep(Duration::from_secs(5));
    handle.join().unwrap();
}

// let a = 3;
// println!("a before drop is {}", a);
// drop(a);
// println!("a after drop is {}", a);

// let handle = thread::spawn(|| {
//     for i in 1..7 {
//         println!("first loop {} iteration", i);
//         thread::sleep(Duration::from_millis(1));
//     }
// });
// handle.join().unwrap();
// for j in 1..4 {
//     println!("second loop {} iteration", j);
//     thread::sleep(Duration::from_millis(1));
// }
// }
