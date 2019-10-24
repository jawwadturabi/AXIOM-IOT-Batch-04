#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 5,
        height: 3,
    };
    let rect2 = Rectangle {
        width: 4,
        height: 2,
    };

    // impl Rectangle {
    //     fn area_of_rectangle(self) -> u32 {
    //         self.height * self.width
    //     }

    //     fn status(self) {
    //         println!("{}  ", self.height);
    //     }
    // }
    // rect1.status();
    // rect2.status()

    // make a struct of cars print model of car1 and companyName
    // of car 2 using methods

    impl Rectangle {
        fn rt(self) {}
        fn area(h: u32, w: u32) -> u32 {
            h * w
        }
    }
    let a = Rectangle::area(6, 12);
    println!("area is : {}", a)
}
