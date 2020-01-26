fn main() {
    for curr in 0..8 {
        let next = (curr + 1) % 8;
        println!("next is : {}", next);
    }
}
