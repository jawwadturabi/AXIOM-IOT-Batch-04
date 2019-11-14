pub trait Average {
    fn _average(&self) -> i32;

    fn name(&self) -> String {
        String::from("scorecard")
    }
}



impl Average for Student {
    fn _average(&self) -> i32 {
        (self.english + self.urdu + self.maths) / 3
    }
}
impl Average for Sportsman {
    fn _average(&self) -> i32 {
        (self.match1 + self.match2 + self.match3) / 3
    }
}
impl Display for    
pub struct Student {
    pub english: i32,
    pub urdu: i32,
    pub maths: i32,
}
pub struct Sportsman {
    pub match1: i32,
    pub match2: i32,
    pub match3: i32,
}
    let stu1 = Student {
        english: 78,
        urdu: 65,
        maths: 85,
    };
    let sp1 = Sportsman {
        match1: 23,
        match2: 0,
        match3: 56,
    };

println!("average is :{}",stu1.name())
//average is scorecard

fn description(value: impl Average,item:impl Average){
    println!("value is :{}",value._average(),value.name())
}
fn description<T: Average>(value: T,item:T){
    println!("value is :{}",value._average(),value.name())
}
fn description(value: impl Average + Summary,item:impl Average + Summary){
    println!("value is :{}",value._average(),value.name())
}
fn description(value: impl Summary + Average,item: impl Display+clone){
    println!("value is :{}",value._average(),value.name())
}
fn description<T,U>(value:T ,item:U ){
    where T:impl Summary + Average,
          U:impl Display+clone  
    println!("value is :{}",value._average(),value.name())

}
