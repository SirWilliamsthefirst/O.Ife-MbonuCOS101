use std::io::Read;
use std::io;

fn main() {

    println!("what is your status in the company?");
    println!("
        if administartor(press a)
        project manager(press p)
        employee(press e)
        customer(press c)
        vendor(press v) ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("unable to read_line");
    let status = input.trim();




    if status == "a" {
    let mut file = std::fs::File::open("globacom_dbase.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}",contents);
}else if status == "p" {
    let mut file = std::fs::File::open("projecttb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}",contents);
}else if status == "e" {
    let mut file = std::fs::File::open("staff_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}",contents);
}else if status == "c"{
    let mut file = std::fs::File::open("customer_tabletb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}",contents);
}else if status == "v" {
    let mut file = std::fs::File::open("dataplan_tabletb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}",contents);
}else {
    println!("incorrect input");

}

}