
use std::io;


fn combo(a:i32,b:i32){
    let sum = a + b ;

    println!("the sum of the values is {}",sum);
}

fn main() {
    let mut input1 = String::new();
    println!("input the value of a");
    io::stdin().read_line(&mut input1).expect("failed to input value");
    let a:i32 = input1.trim().parse().expect("input failed");

    let mut input2 = String::new();
    println!("input the value of b");
    io::stdin().read_line(&mut input2).expect("failed to input value");
    let b:i32 = input2.trim().parse().expect("input failed");

    combo(a,b);
}
