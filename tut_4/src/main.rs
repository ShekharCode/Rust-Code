fn main() {
    // i8 -> -2^7 to 2^7-1 => 128 - 127
    // i16
    // i32
    // i64
    let x:i32 = 2;
    println!("Value of x is {}",x);
    //u8 -> 0 to 2^8 -1 => 0 to 255
    let x:u32 = 972;
    println!("value of x is {}",x);

    let floating_point: f32 = 10.92;
    println!("Value of floating_point is {}",floating_point);
    let floating_point: f64 = 110.78;
    println!("Value of floating_point is {}",floating_point);

    let true_or_false:bool = false;
    println!("{}",true_or_false);
    let true_or_false:bool = true;
    println!("{}",true_or_false);

    let letter:char = 'F';
    println!("{}",letter);

    //tuple
    let tup:(i32,bool,char)= (1,true,'s');
    println!("{}",tup.1);

    let mut tup1:(i64,char,bool) = (34,'d',true);
    println!("{}",tup1.0);
    tup1 = (456,'h',false);
    println!("{}",tup1.2);

    //array
    let mut arr:[i32;5] = [10,20,30,40,50]; // array must be initialised initially
    arr[4] = 100;
    println!("{}",arr[4]); 
    let x:u8 = 4;
    let y = x;
    println!("{} {}",x,y);
}
