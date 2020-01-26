fn main() {
    let mut num: i64 = 0x7ffc4ea2a26c;
    // let r1 = num as *mut i64;
    println!("{}", num);
    // let r2 = r1 as i64;
    // let mut a = 5;
    // let b = &a as *const i32;
    let c = num as *mut i32;
    // let d = &a;
    // println!("{:p}", d);

    // let r2 = &mut num;
    // unsafe {
    //     *c += 1;
    //     println!("{:?}", num);
    // }
}
