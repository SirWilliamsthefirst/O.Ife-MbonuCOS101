use std::io;
use std::io::Write;

// Code for Login Bit
struct LoginInfo {
    username: String,
    password: String,
}

impl LoginInfo {
    // Method to check if the password is valid
    fn info_check(&mut self) -> bool {
        let mut valid_password = false;

        // Constants for password length
        const MAX_PASSWORD_LENGTH: usize = 8;
        const MIN_PASSWORD_LENGTH: usize = 3;

        let upper_case_present = self.password != self.password.to_lowercase();
        let forbidden_char_present = self.forbidden_charachter_check();
        let valid_length =
            self.password.len() >= MIN_PASSWORD_LENGTH && self.password.len() <= MAX_PASSWORD_LENGTH;
        let number_present = self.number_present_check();
        let letter_present = self.password.to_lowercase() != self.password.to_uppercase();

        if !upper_case_present && !forbidden_char_present && valid_length && number_present && letter_present {
            valid_password = true;
        }

        let errors = (
            upper_case_present,
            forbidden_char_present,
            self.password.len() >= MIN_PASSWORD_LENGTH,
            self.password.len() <= MAX_PASSWORD_LENGTH,
            !number_present,
            !letter_present,
        );

        // Print specific error messages based on conditions
        match errors {
            (true, _, _, _, _, _) => println!("Password cannot have uppercase letters"),
            (_, true, _, _, _, _) => println!("Password cannot contain characters '$', '#', or '@'"),
            (_, _, false, _, _, _) => println!("Password is too short"),
            (_, _, true, false, _, _) => println!("Password is too long"),
            (_, _, _, _, false, _) => println!("Password must contain a number"),
            (_, _, _, _, _, false) => println!("Password must contain a letter"),
            (false, false, true, true, true, true) => println!("Login Successful"),
        }

        valid_password
    }

    // Logic for forbidden character check
    fn forbidden_charachter_check(&mut self) -> bool {
        let forbidden_charachters = ["$", "#", "@"];
        let mut status = false;

        for charachter in forbidden_charachters {
            if self.password.contains(charachter) {
                status = true;
                break;
            }
        }

        status
    }

    // Check if a number is present in the password
    fn number_present_check(&mut self) -> bool {
        let numbers = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
        let mut status = false;

        for number in numbers {
            if self.password.contains(number) {
                status = true;
                break;
            }
        }

        status
    }
}

// Function to handle the login process
fn login() {
    let mut username_input = String::new();
    let mut password_input = String::new();
    let mut valid = false;
    const USERNAME_LENGTH: usize = 4;

    let mut login = LoginInfo {
        username: String::new(),
        password: String::new(),
    };

    // Loop to enter the company name and reject until a valid name (greater than 4 characters) is entered
    loop {
        println!("Enter Company Name");
        io::stdin()
            .read_line(&mut username_input)
            .expect("Failed to read line");

        if username_input.trim().len() < 4 {
            println!("Company name too short. Must be at least 4 characters");
            username_input = String::new();
        } else {
            break;
        }
    }

    // Loop to input the password and call functions to validate the password
    while !valid {
        println!("Enter Password");
        io::stdin()
            .read_line(&mut password_input)
            .expect("Failed to read line");

        login = LoginInfo {
            username: username_input[..USERNAME_LENGTH].trim().to_string(),
            password: password_input.trim().to_string(),
        };

        valid = login.info_check();

        // Reset password input if validation fails
        if !valid {
            password_input = String::new();
        }
    }

    // Print the validated username and password
    println!("Username: {}\nPassword: {}", login.username, login.password);
}

// Code for file creation
fn file_creator() {
    let company_names = [
        "Cadbury Nigeria Plc.",
        "Champion Breweries Plc.",
        "Dangote Sugar Refinery Plc.",
        "Flour Mills Nigeria Plc",
        "Nestle Nigeria Plc",
        "Unilever Nigeria Plc",
        "Honeywell Nigeria Plc",
        "Nigeria Breweries Plc",
    ];
    let company_shares = [15_000_000, 25_000_000, 18_000_000, 32_000_000, 8_000_000, 37_000_000, 34_000_000, 30_000_000];
    let company_liabilities = [5_500_000, 8_000_000, 10_000_000, 4_000_000, 1_500_000, 11_000_000, 9_000_000, 12_000_000];
    let year_of_formation = [1965, 1974, 1970, 1960, 1961, 1923, 1906, 1946];
    let mut company_leverages = vec![];

    // Calculate each company's leverage and push it to the leverage vector
    for index in 0..company_names.len() {
        let leverage = (company_shares[index] - company_liabilities[index]) as f32 / company_shares[index] as f32;
        company_leverages.push(leverage * 100.);
    }

    let mut file = std::fs::File::create("Company_Data.xlm").expect("Failed to create file");

    let column_names = ["Company", "Company's Year of Creation", "Company's Shares", "Company's Liabilities", "Company's Leverage"];

    // Write headers of the columns into the file on each column
    for column in column_names {
        file.write_all(format!("{}\t", column).as_bytes()).expect("Failed to write to file");
    }
    file.write_all(format!("\n").as_bytes()).expect("Failed to create new line");

    // Place each company's data on a new row
    for index in 0..column_names.len() {
        file.write_all(
            format!(
                "{}\t {}\t {}\t {}\t {}%\t \n ",
                company_names[index],
                year_of_formation[index],
                company_shares[index],
                company_liabilities[index],
                company_leverages[index]
            )
            .as_bytes(),
        )
        .expect("Failed to write to file");
    }

    const TARGET_LIABILITIES: i32 = 10_000_000;
    const TARGET_SHARES: i32 = 20_000_000;
    const PERCENTAGE_LEVERAGE_MULTIPLIER: f32 = 0.05;

        let mut file =
        std::fs::File::create("Shares-Leverage_Multiples.xlm").expect("Failed to create file");
    file.write_all(format!("Company Name\t Shares-Leverage_Multiple\n").as_bytes()).expect("Failed to write to file");

    // If a company has shares greater than the target share value, save the Shares as a multiple of the percentage leverage
    for pointer in 0..company_names.len() {
        if company_shares[pointer] > TARGET_SHARES {
            file.write_all(
                format!(
                    "{}\t {}\n",
                    company_names[pointer],
                    (company_shares[pointer] as f32 * company_leverages[pointer] as f32) / 100.
                )
                .as_bytes(),
            )
            .expect("Failed to write to file");
        }
    }

    let mut file =
        std::fs::File::create("Shares-Leverage_Multiples.xlm").expect("Failed to create file");
    file.write_all(format!("Company Name\t {}% of Leverage\n", PERCENTAGE_LEVERAGE_MULTIPLIER * 100.)
        .as_bytes())
        .expect("Failed to write to file");

    // If a company's liability is less than the target, multiply its leverage by the target percentage leverage multiplier
    for pointer in 0..company_names.len() {
        if company_liabilities[pointer] < TARGET_LIABILITIES {
            file.write_all(
                format!(
                    "{}\t {}\n",
                    company_names[pointer],
                    (company_leverages[pointer] as f32 * PERCENTAGE_LEVERAGE_MULTIPLIER)
                )
                .as_bytes(),
            )
            .expect("Failed to write to file");
        }
    }
}

// Main function to call other functions
fn main() {
    login();
    file_creator();
}

