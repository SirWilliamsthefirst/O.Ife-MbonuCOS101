fn main() {
    let fullname = "Ife-mbonu Bill Odinaka";
    let department = "Computer Science";
    let uni = "Pan-Atlantic University";

    let mut school = "school of science".to_string();
    //push string
    school.push_str("and technology");

    println!("My name is {}",fullname);
    //check lenght
    println!("My name is {}",fullname.len());
    println!("I am a student of {} department",department);
    println!("{}",school);
    println!("{}",uni);
}
