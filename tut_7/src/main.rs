fn main() {

    //Conditions
    let cond = 2 <=3;
    println!("{}",cond);

    let cond1 = (2 as f32) <= 2.2;
    println!("{}",cond1);

    //Compund Conditions -> &&, || , !
    // Order of precedence - > !, && , || 
    let cond2 = false && cond1;
    println!("{}",cond2);

    let cond3 = true || cond1;
    println!("{}",cond3);

    let cond4 = !true;
    println!("{}",cond4);

    //Control flow
    let food = "Bread";
    if food == "cookie"{
        println!("Yummy")
    }
    else if food == "chocolate"{
        println!("Tasty")
    }
    else{
        println!("Yuckkk...")
    }

}
