// use std::io;
// #[derive(Debug)]
// enum Currencies {
//     USD,
//     Aed,
//     Yen
// }
// fn converter(curr:Currencies)->i32{
//         match curr {
//             Currencies::USD => 156,
//             Currencies::Aed => 43,
//             Currencies::Yen=>22
//         }
//     }
// fn main() {

//     let convert = converter(Currencies::Aed);//156
//     println!("Rs.1 = د.إ{}",convert)
    
// }

struct person_name{
    first:String,
    middle:Option<String>,
    last:String
}
fn main(){
    let person1 =person_name{
        first:String::from("Ali"),
        middle:Some(String::from("Ahmed")),
        last:String::from("Shabbir"),
    };

    let person2=person_name{
        first:String::from("Aslam"),
        middle:None,
        last:String::from("Yousuf"),
    };
    println!("middle name of person is :{:?}",match person2.middle {
        Some(a) => a,
        None => String::from("not exist"),
    })



// middle name of 2nd person is {:?} ",person1.middle,person2.middle
}




    // let a:Option<i8> =Some(3);
    // println!("a is {:?}",a);
    // let b:i8 =5;
    // println!("a after match is {:?}",match a {
    //     Some(x) => x,
    //     None => 0,
    // })


    // let x = [
    //     (0.0065, "USD"),
    //     (0.024, "AED")
    // ];
    // println!("Enter Currency");


    // let mut guess = String::new();
    // io::stdin().read_line(&mut guess).expect("Failed to read line");

    // match guess {
    //     "AED" => println!("AED: {}", 0.0065),
    //     "USD" => println!("AED: {}", 0.024),
    // }


    // println!("\nYour Guessed: {}", x[0].0);
