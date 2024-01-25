use std::io;

fn main() {
    let mut n = String::new();
    println!("what mulitple of n do you need..");
    io::stdin().read_line(&mut n).expect("unable to integer");
    let n:i32 = n.trim().parse().expect("failed input");

    for x in 1..(n+1){
        let multiple = 1 * n;
        println!(" {} * {} = {}",x,n,multiple);
    }

}
