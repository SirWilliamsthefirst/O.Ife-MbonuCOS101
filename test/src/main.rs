
use std::io;

fn main() {
    
    while let patients_counts = 100.00{


    println!("what is your full name?");
    let mut fullname = String::new();
    io::stdin().read_line(&mut fullname).expect("failed to read first name");

    
    println!("what is your date of birth?");
    let mut date_of_birth = String::new();
    io::stdin().read_line(&mut date_of_birth).expect("failed to read data of birth");

    
    println!("what is your email address?");
    let mut email = String::new();
    io::stdin().read_line(&mut email).expect("failed to read email");
    

    println!("what is your phone number?");
    let mut phone_number = String::new();
    io::stdin().read_line(&mut phone_number).expect("failed to read phone_number");
    phone_number = phone_number.trim().parse().expect("failed to input phone_number");

    println!("what is the  number of siblings you have?");
    let mut number_of_siblings = String::new();
    io::stdin().read_line(&mut number_of_siblings).expect("failed to read number_of_siblings");
    let number_of_siblings:i32 = number_of_siblings.trim().parse().expect("failed to input number_of_siblings");

    println!("what is the  number of children you have?");
    let mut number_of_children = String::new();
    io::stdin().read_line(&mut number_of_children).expect("failed to read number_of_children");
    let number_of_children:i32 = number_of_children.trim().parse().expect("failed to input number_of_children");

    println!("what is your medical diagnosis?");
    let mut medical_diagnosis = String::new();
    io::stdin().read_line(&mut medical_diagnosis).expect("failed to read medical_diagnosis (NOTE:'In Small Caps')");


    println!("what is your village or residence?");
    let mut residence = String::new();
    io::stdin().read_line(&mut residence).expect("failed to read residence");


    println!("what is your age?");
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("failed to read first name");
    let age:i32 = age.trim().parse().expect("failed to input first name");

    println!("fullname:{}\nage:{}\ndate of birth:{}\nemail address:{}\nphone number:{}\nnumber of siblings:{}\nnumber of children:{}\nmedical diagnosis:{}\nvillage or residence:{}",fullname,age,date_of_birth,email,phone_number,number_of_siblings,number_of_children,medical_diagnosis,residence);


    println!("
    health medical_diagnosis  amount(#)      village     discount
    Alzheimer                 1,200,000      Akpabom        20%
    Arrhythmla                  550,000      ngbauji         5%
    Chronic kidney disease    1,500,000      Atabrikang      15%
    Diabetes                    800,000      Okorobilom      10%
    Arthritis                   450,000      Emeremen        10%

    ");


if medical_diagnosis == "alzheimer" && age >= 50 && number_of_children > 4 && residence == "akpabom"{
    let x:f64 = 1200000.0*0.80;
    println!("The amount is {}",x);
}else if medical_diagnosis == "arrythmia" && age == 30 && number_of_siblings > 4 && residence == "ngbauji" {
    let y:f64 = 550000.0*0.95;
    println!("The amount is #{}",y);

}else if medical_diagnosis == "chronic kidney disease" && age > 40 && number_of_children > 3 &&number_of_siblings >3 && residence == "atabrikang"{
    let z:f64 = 1500000.0*0.85;
    println!("The amount is #{}",z);

}else if medical_diagnosis == "diabetes" && age > 28 && age < 45 && number_of_children >= 2 && number_of_children <= 4 && residence == "okorobilom"{

    let p:f64 = 800000.0*0.90;
    println!("The amount is #{}",p);
}else if medical_diagnosis == "arthritis" && age > 58 && number_of_siblings > 5 && number_of_children > 5 && residence == "emeremen" {

    let m:f64 = 4500000.0*0.95;
    println!("The amount is #{}",m);
}else {
    println!("No of your condition fit in the discription ");
}

    }
    
    }
