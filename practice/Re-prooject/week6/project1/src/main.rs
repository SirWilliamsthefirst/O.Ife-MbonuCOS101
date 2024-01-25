
use std::io;

fn main() {
    let mut candidate = 0;

    loop {
        if candidate >= 150 {
            break; // Exit the loop when 150 candidates have been processed
        }

        let mut input1 = String::new();
        let mut input2 = String::new();
        let mut input3 = String::new();
        let mut input4 = String::new();
        let mut input5 = String::new();
        let mut input6 = String::new();
        let mut input7 = String::new();

        println!("Are you the current class captain?...(yes/no)");
        io::stdin().read_line(&mut input1).expect("Unable to read input");

        println!("Are you in level 100?...(yes/no)");
        io::stdin().read_line(&mut input2).expect("Unable to read input");

        println!("Is your CGPA above 4.0?...(yes/no)");
        io::stdin().read_line(&mut input3).expect("Unable to read input");

        println!("What is your name? (Please input your full name)");
        io::stdin().read_line(&mut input4).expect("Unable to read input");

        println!("Input your email...");
        io::stdin().read_line(&mut input5).expect("Unable to read input");

        println!("What department are you in?");
        io::stdin().read_line(&mut input6).expect("Unable to read input");

        println!("Input your state of origin...");
        io::stdin().read_line(&mut input7).expect("Unable to read input");

        // Trim the input strings to remove whitespaces
        let input1 = input1.trim();
        let input2 = input2.trim();
        let input3 = input3.trim();

        if input1 == "yes" && input2 == "no" && input3 == "yes" {
            println!(
                "\nName: {}\nEmail: {}\nDepartment: {}\nState of origin: {}",
                input4.trim(),
                input5.trim(),
                input6.trim(),
                input7.trim()
            );
            println!("You are eligible to vote.");
        } else {
            println!("Sorry, you are not eligible to vote.");
        }

        candidate += 1;
    }
}






/*
using while loop

use std::io;

fn main() {
    let mut candidate = 0;

    while candidate <150{
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();
    let mut input5 = String::new();
    let mut input6 = String::new();
    let mut input7 = String::new();

    println!("Are you the current class captain?...(yes/no)");
    io::stdin().read_line(&mut input1).expect("Unable to read input");

    println!("Are you in level 100?...(yes/no)");
    io::stdin().read_line(&mut input2).expect("Unable to read input");

    println!("Is your CGPA above 4.0?...(yes/no)");
    io::stdin().read_line(&mut input3).expect("Unable to read input");

    println!("What is your name? (Please input your full name)");
    io::stdin().read_line(&mut input4).expect("Unable to read input");

    println!("Input your email...");
    io::stdin().read_line(&mut input5).expect("Unable to read input");

    println!("What department are you in?");
    io::stdin().read_line(&mut input6).expect("Unable to read input");

    println!("Input your state of origin...");
    io::stdin().read_line(&mut input7).expect("Unable to read input");

    // Trim the input strings to remove whitespaces
    let input1 = input1.trim();
    let input2 = input2.trim();
    let input3 = input3.trim();

    if input1 == "yes" && input2 == "no" && input3 == "yes" {
        println!(
            "\nName: {}\nEmail: {}\nDepartment: {}\nState of origin: {}",
            input4.trim(),
            input5.trim(),
            input6.trim(),
            input7.trim()
        );
        println!("You are eligible to vote.");
    } else {
        println!("Sorry, you are not eligible to vote.");
    }
}
candidate += 1;
}

*/




















//using for statement

/*use std::io;

fn main() {
    for _ in 0..150{
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();
    let mut input5 = String::new();
    let mut input6 = String::new();
    let mut input7 = String::new();

    println!("Are you the current class captain?...(yes/no)");
    io::stdin().read_line(&mut input1).expect("Unable to read input");

    println!("Are you in level 100?...(yes/no)");
    io::stdin().read_line(&mut input2).expect("Unable to read input");

    println!("Is your CGPA above 4.0?...(yes/no)");
    io::stdin().read_line(&mut input3).expect("Unable to read input");

    println!("What is your name? (Please input your full name)");
    io::stdin().read_line(&mut input4).expect("Unable to read input");

    println!("Input your email...");
    io::stdin().read_line(&mut input5).expect("Unable to read input");

    println!("What department are you in?");
    io::stdin().read_line(&mut input6).expect("Unable to read input");

    println!("Input your state of origin...");
    io::stdin().read_line(&mut input7).expect("Unable to read input");

    // Trim the input strings to remove whitespaces
    let input1 = input1.trim();
    let input2 = input2.trim();
    let input3 = input3.trim();

    if input1 == "yes" && input2 == "no" && input3 == "yes" {
        println!(
            "\nName: {}\nEmail: {}\nDepartment: {}\nState of origin: {}",
            input4.trim(),
            input5.trim(),
            input6.trim(),
            input7.trim()
        );
        println!("You are eligible to vote.");
    } else {
        println!("Sorry, you are not eligible to vote.");
    }
}

}*/