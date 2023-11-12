/*Rust program to read the hieght of a person
and then print if person is tall,dwarf,
or average height person*/

use std::io;

fn main() {
    let mut input = String::new();

    println!("\nEnter your hieght (in centimeters):");
    io::stdin().read_line(&mut input).expect("not a valid string");
    let height:f32 = input.trim().parse().expect("not a valid string");

    if height >= 150.0 && height <=170.0{
        println!("you are of averge height person");
    } else if height > 170.0 && height >=195.0{
        println!("You are tall");
    } else if height <150.0 && height >100.0{
        println!("You are dwarf");
    } else{
        println!("Abnormal height");
    }
}