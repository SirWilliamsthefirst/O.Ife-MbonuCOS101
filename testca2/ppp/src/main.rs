use std::io::Write;
use std::io;
use std::io::prelude::*;

fn username() {

    println!("Note: The first four(4) letters of your respective company's name should be in your Username");
    println!("Input Username: ");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let user_name:String = input1.trim().parse().expect("Invalid input");

    if user_name.len() >= 3 && user_name.len() < 9 {
        println!("The length of your Username is within security bounds ");
    }
    else {
        println!("The length of your Username is not within security bounds. Please increase or decrease the length");
        return;
    }

    println!("Enter Password: ");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Failed to read input");
    let password = input2.trim();

    let security_bounds = vec![
        "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r",
        "s", "t", "u", "v", "w", "x", "y", "z", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9",
    ];
    
    let mut is_within_bounds = true;
    for c in password.chars() {
        if !security_bounds.contains(&c.to_string().as_str()) {
            is_within_bounds = false;
            break;
        }
    }
    if is_within_bounds {
        println!("Password is within security bounds");
    } 
    else {
        println!("Password is not within security bounds. Please choose a different password.");
    }
}

fn company() {

    let comapny_name = [
        "\tCOMPANY NAAMES", 
        "\n1) Cadbury Nigeria Plc",
        "\n2) Champion Breweries Plc",
        "\n3) Dangote Sugar Refinery Plc",
        "\n4) Flour Mills Nigeria Plc", 
        "\n5) Nestle Nigeria Plc", 
        "\n6) Unilever Nigeria Plc", 
        "\n7) Honeywell Nigeria Plc", 
        "\n8) Nigerain Breweries Plc"
    ];

    let company_year = [
        "\t\tCOMPANY FOUNDING YEARS", 
        "\t 1965", 
        "\t 1974", 
        "\t 1970", 
        "\t 1960", 
        "\t 1961", 
        "\t 1923", 
        "\t 1906", 
        "\t 1946"
    ];

    let company_shares = [
        "\t\tCOMPANY SHARES($)", 
        "\t 15,000,000", 
        "\t 25,000,000", 
        "\t 18,000,000", 
        "\t 32,000,000", 
        "\t 8,000,000", 
        "\t 37,000,000", 
        "\t 34,000,000", 
        "\t 30,000,000"
    ];

    let company_liabilities = [
        "\t\tCOMPANY LIABILITIES($)", 
        "\t 5,500,000", 
        "\t 8,000,000", 
        "\t 10,000,000", 
        "\t 4,000,00", 
        "\t 1,500,000", 
        "\t 11,000,000", 
        "\t 9,000,000", 
        "\t 12,000,000"
    ];

    let company_percentage_leverages = [
        "\t\tCOMPANY PERCENTAGE LEVERAGES(%)", 
        "\t 63.33", 
        "\t 68", 
        "\t 44.44", 
        "\t 87.50", 
        "\t 81.25", 
        "\t 70.27", 
        "\t 73.53", 
        "\t 60"
    ];

    let mut file = std::fs::File::create("List of Nigerian Companies and their relevant information.txt").expect("create failed");

    for x in 0..9{

        file.write_all(comapny_name[x].as_bytes()).expect("Failed to write");

        file.write_all(company_year[x].as_bytes()).expect("Failed to write");

        file.write_all(company_shares[x].as_bytes()).expect("Failed to write");
 
        file.write_all(company_liabilities[x].as_bytes()).expect("Failed to write");
 
        file.write_all(company_percentage_leverages[x].as_bytes()).expect("Failed to write");

    }
    println!("\nData written to file.");

}

fn percentage() {

    let comapny_name_1 = [
        "\tCOMPANY NAAMES", 
        "\n1) Champion Breweries Plc", 
        "\n2) Flour Mills Nigeria Plc", 
        "\n3) Unilever Nigeria Plc", 
        "\n4) Honeywell Nigeria Plc", 
        "\n5) Nigerain Breweries Plc"
    ];

    let company_percentage_leverages_1 = [
        "\t\tCOMPANY PERCENTAGE LEVERAGES(%)", 
        "\t 68", 
        "\t 87.50", 
        "\t 70.27", 
        "\t 73.53", 
        "\t 60"
    ];

    let mut file = std::fs::File::create("Companies who's shares are greater than 20 million and their percentage leverages.txt").expect("create failed");

    for x in 0..6{

        file.write_all(comapny_name_1[x].as_bytes()).expect("Failed to write");

        file.write_all(company_percentage_leverages_1[x].as_bytes()).expect("Failed to write");
    }
    println!("\nData written to file.");
    
}

struct Company {
    Cadbury:f64, Champion:f64, Flour:f64, Nestle:f64, Honeywell:f64
}

fn liability() {

    let total = Company {
        Cadbury: 0.05 * 68.00,
        Champion: 0.05 * 87.50,
        Flour: 0.05 * 70.27,
        Nestle: 0.05 * 73.53,
        Honeywell: 0.05 * 60.00,
    };
    println!("The percentage leverage of Cadbury Nigeria Plc is {}",total.Cadbury);
    println!("The Percentage leverage of Champion Breweries Plc is {}",total.Champion);
    println!("The percentage leverage of Flour Mills Nigeria Plc is {}",total.Flour);
    println!("The Percentage leverage of Nestle Nigeria Plc is {}",total.Nestle);
    println!("The Percentage leverage of Honeywell Nigeria Plc is {}",total.Honeywell);
   
}

fn main() {

    println!("This program runs all the codes and their functions");
    username();
    company();
    percentage();
    liability();
}