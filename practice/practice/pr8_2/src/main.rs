use std::io;
fn fame() {
    let mut henry = String::new();
    io::stdin().read_line(&mut henry).expect("unable to input");
    let henry:i64 = henry.trim().parse().expect("input failed");


    println!("The number is {}",henry);
}

fn main() {
    println!("Guess any number!");
    fame();
}
