

use std::io;

fn main() {
    let candidate = 0;

    while candidate  <= 150{

    println!("what is your full name?");
    let mut fullname = String::new();
    io::stdin().read_line(&mut fullname).expect("failed to read first name");


    println!("what is your email address?");
    let mut email = String::new();
    io::stdin().read_line(&mut email).expect("failed to read email");
    
    println!("what is your department?");
    let mut department = String::new();
    io::stdin().read_line(&mut department).expect("failed to read email");
     

    println!("what is your state of origin?");
    let mut state  = String::new();
    io::stdin().read_line(&mut state).expect("failed to read email");
     

    println!("Are you the current Class Rep?(yes or no)");
    let mut class_rep  = String::new();
    io::stdin().read_line(&mut class_rep).expect("failed to read email");


    println!("input your current level");
    let mut level  = String::new();
    io::stdin().read_line(&mut level).expect("failed to read email");
    let level:i32 = level.trim().parse().expect("Error");


    println!("input your current CGPA");
    let mut gpa  = String::new();
    io::stdin().read_line(&mut gpa).expect("failed to read email");
    let gpa:f32 = gpa.trim().parse().expect("Error");



    if class_rep.trim() == "yes" && level > 100 && level <= 400 && gpa > 4.0 {
        println!("Name:{}\nEmail:{}\nDepartment:{}\nState of Origin:{}",fullname,email,department,state);
        println!("you can vote");
    }else {
        println!("Sorry,you are not eligible");
    }


}    
    }
