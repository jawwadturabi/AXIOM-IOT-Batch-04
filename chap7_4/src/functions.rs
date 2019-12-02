pub mod calc{
    pub  fn add(num1: i32,num2:i32){
        println!("addition of {} and {} is : {}",num1,num2,num1+num2)
    }
    pub  fn sub(num1: i32,num2:i32){
        println!("subtraction of {} and {} is : {}",num1,num2,num1-num2)
    }   
}

pub mod adv_calc{
    pub fn square(val: i32){
        println!("square of {} is :{}",val,val*val);
    }
}
