fn main() {
    //create an emptpy vector "city"
    let mut city : Vec<String> = Vec::new();
    //print city vector
    println!("The city vector has ethe vectorlement {}",city.len());
    //push new element into 
    let mut input1 = String::new(); 
    println!("How many city do you want to enter");
    std::io::stdin().read_line(&mut input1).expect("failed to read input1");
    let city_num:i32 = input1.trim().parse().expect("Invalid input");
    for count in 0..city_num{
        let mut input2 = String::new();
        println!("Enter City {}",count+1);
        std::io::stdin().read_line().expect("Failed to read input");
        let new_city:String = input2.trim().parse().expect("Invalid input");
        city.push(new_city);

    }

    print!("your preferred city are:\n");
    let mut count=1;
    //loop to iterate element in vector
    for i in city{
        //iterating through i on vector
        println!("{} {}",count,i);
    }
}
