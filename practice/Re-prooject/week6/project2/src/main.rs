

use std::io;

fn main(){

    for _ in 0..500{

    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Input your name...");
    io::stdin().read_line(&mut input1).expect("failed to read input");

    println!("How many papers have been published?");
    io::stdin().read_line(&mut input2).expect("failed to read input");
    let number:i32 = input2.trim().parse().expect("unable to input");

    let mut name = input1.trim();

    if number >= 3 && number <= 5 {
        println!("Your incentive is #500,000");
    } else if number > 5 && number < 10 {
        println!("Your incentive is #800,000");
    } else if number >= 3 && number <= 5 {
        println!("Your incentive is #500,000");
    } else if number >= 10  {
        println!("Your incentive is #1,000,000");
    } else if number < 3 {
        println!("Your incentive is #100,000");
    } else{
        println!("invalide information");
    }
}

}
