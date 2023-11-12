use std::io;

fn main() {
    println!("Enter value of 'a'");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("invalid information");
    let  a:f32 = input1.trim().parse().expect("failure to input");

    println!("Enter value of 'b'");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("invalid information");
    let  b:f32 = input2.trim().parse().expect("failure to input");

    println!("Enter value of 'c'");
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("invalid information");
    let  c:f32 = input3.trim().parse().expect("failure to input");

    if (b.powf(2.0))-4.0*(a*c) < 0.0{

        println!("The answer is imaginary")
    }

    if (b.powf(2.0))-4.0*(a*c) > 0.0{
        let answer1:f32= (-b + (b.powf(2.0)-(4.0 *a*c)).sqrt())/2.0*a;
       let answer2:f32= (-b - (b.powf(2.0)-(4.0 *a*c)).sqrt())/2.0*a;

       println!("The roots of the equation are {} & {}",answer1,answer2);

    }
    if (b.powf(2.0))-4.0*(a*c) == 0.0{
        let answer1:f32= (-b + (b.powf(2.0)-(4.0 *a*c)).sqrt())/2.0*a;
        println!("The roots of the equation are {}",answer1);
    }
}

