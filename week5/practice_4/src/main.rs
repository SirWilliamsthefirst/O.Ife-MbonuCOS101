//Rust program to determine age pass

use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter your name");
    io::stdin().read_line(&mut input1).expect("not valid string");

    println!("Enter age");
    io::stdin().read_line(&mut input2).expect("not valid string");
    let age:i32 = input2.trim().parse().expect("Not a valid number");

    if age >=18 {
        println!("Welcome to the party:{}",input1);
    } else{
        println!("Oops,you are not of age to enter the party{}",input1);
    }

}