    fn main(){
    let name = "Aish Lawal";
    let uni:&str = "Pan-Alantic University";
    let addr:&str = "km 52 lekki-Epe Expressway, Ibejulekki,lagos";
    println!("Name: {}",name);
    println!("university: {},\nAddress: {}",uni,addr);

    let department:&'static str =  "Computer science";
    let school:&'static str = "School of science and technology";
    println!("department: {},\nSchool: {}",department,school);
}b