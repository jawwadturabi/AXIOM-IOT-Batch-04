#[derive(Debug)]
enum Currency {
    Pkr,
    usDollar,
    Yen,
    Aed,
}

fn main() {
    let num = "hello";
    match  num {
        num => println!("value exist is : {}", num),
    }
    // let amount = Currency::Yen;
    // match amount {
    //     Currency::Pkr => println!("$1 in pkr is Rs.156"),
    //     Currency::usDollar => println!("Rs.1 in us dollar is $0.0156"),
    //     Currency::Yen => println!("Rs.1 in Yen is 22"),
    //     Currency::Aed => println!("Rs.1 in Aed is 43"),
    // }
}
