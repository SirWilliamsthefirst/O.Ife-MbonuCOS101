use std::io;
fn hen(p:i32,q:i32){
    let sum = p + q;
    println!("Sum of A and B = {}", sum);
}
fn main() {
    let mut input1 = String::new();
    println!("Enter input for parameter A:");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let p:i32 = input1.trim().parse().expect("Invalid Input");

    let mut input2 = String::new();
    println!("Enter input for parameter B:");
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let q:i32 = input2.trim().parse().expect("Invalid Input");

    hen(p,q);
}