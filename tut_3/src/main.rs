fn main() {
    let x = 4;
    println!("Value of x is {}",x); // x = 5 -> it throws a error as it cannot assign twice to a variable

    let mut y = 7;
    println!("Value of y is {}",y);
    y = 69;
    println!("Value of y is {}",y);

    let z = 10;
    println!("Value of z is {}",z);
    let z = 20;
    println!("Value of z is {}",z);

    //Name Shadowing
    let a = 10;
    println!("Value of a is {}",a);
    {
        let a = a - 10;
        println!("Value of a is {}",a);
    }
    let a = a+10;
    println!("Value of a is {}",a);

    let b = 4;
    println!("Value of b is {}",b);
    let b = "hello";
    println!("Value of b is {}",b);

    //const
    const SECONDS_IN_MINUTE: u32 = 60; // once created it cannot be redefined or reassigned
    println!("Seconds in a minute {}",SECONDS_IN_MINUTE);
}
