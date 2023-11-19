use std::io;
fn main() {
    let mut number_of_Researchers = String::new();
    println!("How many researches have been inputed");
    io::stdin().read_line(&mut number_of_Researchers).expect("Inavlid Input");
    let number_of_Researchers:i32 = number_of_Researchers.trim().parse().expect("Not a valid number");


    if number_of_Researchers > 500{
        println!("Sorry, the research queue is full");
    }else{
        println!("Enter the reasearcher's name");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Invalid Input");


        let mut number_of_Papers = String::new();
        println!("How many papers has {} produced", name.trim());
        io::stdin().read_line(&mut number_of_Papers).expect("invalid input");
        let number_of_Papers:i32 = number_of_Papers.trim().parse().expect("invalid input");

        if number_of_Papers>= 10{
            println!("{} has an incentive of 1000000.00", name.trim());
        }else if number_of_Papers < 3{
            println!("{} has an incentive of 100000", name.trim());
        }else if number_of_Papers > 5{
            println!("{} has an incentive of 800000", name.trim());
        }else if number_of_Papers <= 5{
            println!("{} has an incentive of 500000", name.trim());
        }
    }



}