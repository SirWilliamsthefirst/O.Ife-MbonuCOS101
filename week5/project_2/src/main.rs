/*A rust program to determine the annual 
incentive for an employee by his/her age and experience*/

use std::io;

fn main() {
    println!("What is your age?");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("failed to read input");
    let age:f32= input1.trim().parse().expect("failed to input");
    
    println!("Are you  experienced?(Yes or No)");
    let mut input2 =String::new();
    io::stdin().read_line(&mut input2).expect("failed to read input");
    let experience = input2.trim();

    if experience == "yes" && age >= 40.0{
        println!("The incentive you as an employee is #1,560,000 per month");
    } else if experience == "yes" && age >= 30.0 && age < 40.0{
        println!("The incentive you as an employee is #1,480,000 per month");
    } else if experience == "yes" && age < 28.0{
        println!("The incentive you as an employee is #1,300,000 per month");
    }else if experience == "No"{
        println!("The incentive you as an employee is #100,000 per month");
    }else {
        println!("you have the nessesary requirement for the job");
    }

}
