fn main() {
    //marksheet
    // for A+,A,B ,C and fail
    // let marks =62;

    // if marks >=70 && marks<80{
    //     println!("A grade")
    // }
    // else if marks >=80 && marks<90{
    //     println!("A+ grade")
    // }
    // else if marks >=60 && marks<70 {
    //     println!("B grade")
    // }
    // else if marks >=50 && marks<60{
    //     println!("C grade")
    // }
    // else{
    //     print!("Fail");
    // }
    for curr in (0..9).rev() {
        let next = (curr - 1) % 8;
        // leds[next].on();
        // delay.delay_ms(60_u8);
        // leds[curr].off();
        // delay.delay_ms(60_u8);
        println!("next is {} and curr is {}", next, curr)
    }
}

// let number = 4.0;
// let calculator = false;
//  if number>3.0 || calculator {
//     println!("number is 3 or greater")
// }
// else {
//     println!("number is less than 3")
// };//10
