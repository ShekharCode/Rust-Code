// creating the functions
fn main() {
    println!("Hello, world!");
    test_one();
    add_numbers(20,30);
    let ans = 20;
    println!("The value of ans is {}",ans);

    let y = 20; //statement
    println!("y is {}",y);

    //expression -> evaluates to something
    let bool_res = 2<3; //2<3 is an expression
    println!("bool_res is  {}",bool_res);
    let number = {
        let x = 3; // statement
        x+1 //expression - >4 is returned to number variable 
    };
    println!("number is {}",number);

    //funtion retuning
    let res = summation(5,5);
    println!("res is {}",res);
}

fn test_one(){
    println!("test has been called ...");
}
fn add_numbers(x:i32,y:i32){
    println!("the sum is {}",x+y);
}
fn summation(x:i32,y:i32)->i32{
    let ans = x+y;
    if ans>= 10 {
        return ans - 10;
    }
    ans                         
}
