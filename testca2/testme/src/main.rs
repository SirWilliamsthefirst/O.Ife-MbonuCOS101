use std::io;
use std::io::Write;

struct LoginInfo{
    username: String,
    password: String
}

impl LoginInfo{
    fn info_check(&mut self)->bool{

        let mut valid_password = false; //initializes the check for if the password is valid to false by default.

        const MAX_PASSWORD_LENGTH:usize = 8;//I created variables for minimum and maximum values for the length of the password.
        const MIN_PASSWORD_LENGTH:usize = 3;

        let upper_case_present = self.password != self.password.to_lowercase();//checks if the original password is different from its lowercase version.

        let forbidden_charachter_present = self.forbidden_charachter_check();//checks to see if Forbidden charachters are present.

        let valid_length = self.password.len() >= MIN_PASSWORD_LENGTH && self.password.len() <= MAX_PASSWORD_LENGTH;//to check the lenght

        let number_present = self.number_present_check();
        
        let letter_present = self.password.to_lowercase() != self.password.to_uppercase(); 

        if !upper_case_present && !forbidden_charachter_present && valid_length && number_present && letter_present{//checks to see if the password fulfils all conditions to be valid
            valid_password = true;//if all conditions are met, password is determined valid.
        }

        let errors = (upper_case_present, forbidden_charachter_present,self.password.len() >= MIN_PASSWORD_LENGTH,self.password.len() <= MAX_PASSWORD_LENGTH, number_present, letter_present);//sets all the possible errors to a tuple so it can be used in the match statement.

        match errors {//match statement to print out the specific error of the password. If there are no errors then it prints out login succesful. 

            (true,_,_,_,_,_) => println!("Password should not have upper case letters"),
            
            (_,true,_,_,_,_) => println!("Password should not contain charachters '$','#' or '@'"),
            
            (_,_,false,_,_,_) =>println!("password is too short"),
            
            (_,_,true,false,_,_) => println!("Password is too long"),
            
            (_,_,_,_,false,_) =>println!("Password should contain a number"),
            
            (_,_,_,_,_,false) =>println!("Password should contain a letter"),
            
            (false, false, true, true, true,true) => println!("Login Succesful"),
        
        }

        return valid_password;//returns the status of the password.
    }

    fn forbidden_charachter_check(&mut self)->bool{ //for forbidden charachtrer check
        let forbidden_charachters = ["$","#","@"];//The array contains a list of forbidden charachters
        let mut check = false;

        for charachter in forbidden_charachters{//loops of the charachters in the forbidden characters array to checks if the charachter is present in the password.

            if self.password.contains(charachter){
                check = true;
                break;
            }
        }

        return check;
    }

    fn number_present_check(&mut self)->bool{
        let numbers = ["0","1","2","3","4","5","6","7","8","9"];//an array containing only 10 numbers from the instruction.
        let mut check = false;
        for number in numbers{
            if self.password.contains(number){//to check if a number is present in the password
                check = true;
                break;
            }
        }
        return check;
    }
}

fn login(){

    let mut username_input = String::new();
    let mut password_input = String::new();
    let mut valid = false;
    
    const USERNAME_LENGTH:usize = 4; //to avoid "magic numbers", we create a constant and set it to 4 to represents the Username length.

    let mut login = LoginInfo{//intializing a variable to hold the user's information
        username:String::new(),
        password: String::new(),
    };


    loop{//code to enter company name and keep rejecting until a valid name(one greater than 4 charachters is inputed).

        println!("Enter Company Name");
        io::stdin().read_line(&mut username_input).expect("Failed to read line");
        if username_input.trim().len() < 4{
        
        println!("Company name too short. Must be at least 4 Charachters");
        username_input = String::new();
    
    }   else{
            break;
    }

    };

    while !valid{// code to input password and call funtions to validate the password entered
        println!("Enter Password");
        io::stdin().read_line(&mut password_input).expect("Failed to read line");

        login = LoginInfo {
            username: username_input[..USERNAME_LENGTH].trim().to_string(),//Here, to make the username 4 charachters, we slice the string.
            password: password_input.trim().to_string()
        };

        valid = login.info_check();
        if !valid{
            password_input = String::new();
             }
            
    }
    println!("Username: {} \nPassword: {}", login.username, login.password);// prints the username and password after being validated. Not really useful, but helpful.

}





 //FOR THE FILE
    


fn file_creator(){//code to create files.

    let company_names = ["Cadbury Nigeria Plc.", "Champion Breweries Plc." , "Dangote Sugar Refinery Plc.", "Flour Mills Nigeria Plc", "Nestle Nigeria Plc", "Unilever Nigeria Plc", "Honeywell Nigeria Plc", "Nigeria Breweries Plc"];//intialize vectors.

    let company_shares = [15_000_000,25_000_000,18_000_000,32_000_000,8_000_000,37_000_000,34_000_000,30_000_000];

    let company_liabilities = [5_500_000,8_000_000,10_000_000,4_000_000,1_500_000,11_000_000,9_000_000,12_000_000];

    let year_of_formation = [1965,1974,1970,1960,1961,1923,1906,1946];

    let mut company_leverages = vec![];

    for index in 0..company_names.len(){//code to calculate each company's leverage and push it to the leverage vector.

        let leverage = (company_shares[index] - company_liabilities[index])as f32 / company_shares[index] as f32;
    
        company_leverages.push(leverage*100.)//multiples the float by a hundred so it's saved a percentage.;

    };

    let mut file = std::fs::File::create("Company_Data.xlm").expect("Failed to create file");//creates file.

    let column_names = ["Company","Company's Year of Creation", "Company's Shares", "Company's Liabilities","Company's Leverage"]; 

    for column in column_names{
        file.write_all(format!("{}\t", column).as_bytes()).expect("Failed to write to file");
    }
    file.write_all(format!("\n").as_bytes()).expect("Failed to create new line");

    for index in 0..column_names.len(){
        file.write_all(format!("{}\t {}\t {}\t {}\t {}%\t \n ",company_names[index],year_of_formation[index],company_shares[index],company_liabilities[index],company_leverages[index]).as_bytes()).expect("Failed to write to file");//places each company's data on a new row.
    


    const TARGET_LIABILITIES:i32 = 10_000_000; //Constants to avoid Magic numbers in future operations.
    const TARGET_SHARES:i32 = 20_000_000;
    const PERCENTAGE_LEVERAGE_MULTIPLIER:f32 = 0.05;

    let mut file = std::fs::File::create("Shares-Leverage_Multiples.xlm").expect("Failed to create file");//creates file.
    file.write_all(format!("Company Name\t Shares-Leverage_Multiple\n").as_bytes()).expect("Failed to write to file");

   

    for pointer in 0..company_names.len(){

        if company_shares[pointer] > TARGET_SHARES{//If a company has shares greater thatn the Target share value then the Shares are saved as a multiple of the percentage leverage.
            file.write_all(format!("{}\t {}\n", company_names[pointer], (company_shares[pointer]as f32* company_leverages[pointer]as f32)/100.).as_bytes()).expect("Failed to write to file"); //recall Leverage was multipled by 100 initially for display purposes. Here, we divide by 100 for calculation purposes.
        }

    }

    let mut file = std::fs::File::create("Shares-Leverage_Multiples.xlm").expect("Failed to create file");//creates file.
    file.write_all(format!("Company Name\t {}% of Leverage\n", PERCENTAGE_LEVERAGE_MULTIPLIER*100.).as_bytes()).expect("Failed to write to file");//Creates columns

    for pointer in 0..company_names.len(){//If a company's liability is less than the target, It's leverage gets multipled by the targer percentage leverage multiplier. As at the time of writing this, it's 5%.

        if company_liabilities[pointer] < TARGET_LIABILITIES{
            file.write_all(format!("{}\t {}\n", company_names[pointer], (company_leverages[pointer] as f32 * PERCENTAGE_LEVERAGE_MULTIPLIER)).as_bytes()).expect("Failed to write to file");
        }

    }
}


fn main() {//this is to call the main functions.
    login();
    file_creator();
}