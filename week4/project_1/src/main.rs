 use std::io;
fn main() {
        let mut distance = String::new();
        let mut time = String::new();

        println!("Enter your distance in kilometers: ");
        io::stdin().read_line(&mut distance).expect("Invalid format");
        let d:f32 = distance.trim().parse().expect("Invalid format");
        println!("Enter yout time in hours: ");
        io::stdin().read_line(&mut time).expect("Invalid");
        let t:f32 = time.trim().parse().expect("Invalid");

        let speed = d/t;
        println!("The speed is {}",speed/0.62137119);

        
}


