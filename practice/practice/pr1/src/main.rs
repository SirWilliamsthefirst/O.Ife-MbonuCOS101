use std::io;

fn main() {
    println!("guess a number...");
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("failed to read line");
    let number:u32 = num.trim().parse().expect("failed to input");

    println!("{} is a great choice!",number);
}
