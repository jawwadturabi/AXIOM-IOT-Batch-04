// use std::cmp::PartialOrds;
// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];

//     for &item in list.iter() {
//         if item > largest {
//             largest = item;
//         }
//     }

//     largest
// }

// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];

//     let result = largest(&number_list);
//     println!("The largest number is {}", result);

//     let char_list = vec!['y', 'm', 'a', 'q'];

//     let result = largest(&char_list);
//     println!("The largest char is {}", result);
// }

// fn main(){
//     let arr =[23,56,90,12];
//         let a =3;
//     for i in &arr.iter(){
//         if a>i{
//             a= i
    
//     }
//     }
//     println!("a is {}",a)
// }

// #[derive(Debug)]
// struct User<T,U>{
//     Name:T,
//     email:U
// }

// #[derive(Debug)]
// enum Option<T> {
//     Some(T),
//     None,
// }

// #[derive(Debug)]
// enum Result<T,E> {
//     Ok(T),
//     Err(E),
// }
// #[derive(Debug)]
// enum Student<T> {
//     Boys(T),
//     Girls(T),
// }

// fn main(){
//     let user1 = User{
//         Name:String::from("ali"),
//         email:12.2
//     };
//     println!("{:#?} ",user1)
// }


// struct Point<T, U> {
//     x: T,
//     y: U,
// }

// impl<T, U> Point<T, U> {
//     fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
//         Point {
//             x: self.x,
//             y: other.y,
//         }
//     }
// }

// fn main() {
//     let p1 = Point { x: 5, y: 10.4 };
//     let p2 = Point { x: "Hello", y: 'c'};

//     let p3 = p1.mixup(p2);

//     println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
// }


// fn largest<T>(list:&[T])->T{
//     let mut largest=list[0];
//     for &i in list.iter(){
//         if i>largest{
//             largest = i
//         }
//     }
//     largest
// }
// use std::cmp::Ordering;
// fn largest<T:std::cmp::Ord>(list: &[T]) -> &T {
//     let mut largest = &list[0];

//     for item in list.iter() {
//         // match item.cmp(&largest) {
//         //     Ordering::Greater => largest = &item,
//         //     Ordering::Less => (),
//         //     Ordering::Equal => (),
//         // }
//         println!("cmp is {:?}",item.cmp(&largest))
//     }
//     largest
// }
// fn main(){
//     let arr =[23,56,90,67];
//     println!("largest number is {}",largest(&arr));

//     let arr2 =['a','u','i','o'];
//     println!("largest character is {}",largest(&arr2));

// }
use std::io;
fn main(){
  loop{
      println!("Enter the number");
      let mut guess=String::new();
    io::stdin().read_line(&mut guess)
    .expect("Failed to read line");

let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(e) =>{
        println!("Invalid input");
        continue} ,
};
  }
// println!("guess is :{}",guess)
}







// fn largest_float(list:&[f32])->f32{
//     let mut largest=list[0];
//     for &i in list.iter(){
//         if i>largest{
//             largest = i
//         }
//     }
//     largest
// }



