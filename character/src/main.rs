fn add(x:f32,y:f32)->(f32,f32,f32,f32){
        (x+y,x-y,x*y,x/y)
}

fn main() {
    let Add = add(2.0,6.0).0;//this is for addition
    
    let sub = add(2.0,6.0).1;    
    let mult = add(2.0,6.0).2;    
    let div = add(2.0,6.0).3;    

    println!("addition is {:?}",Add);
    println!("subtraction is {:?}",sub);
    println!("multiplication is {:?}",mult);
    println!("division is {:?}",div);


}


