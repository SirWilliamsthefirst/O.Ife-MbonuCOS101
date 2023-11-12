//rust program to caculate the price for customers
//and calculate their 5% discount if >#1000 

use std::io;

fn main() {
    /*let p = "Poundo Yam and Edikaiko Soup";
      let f = "Fried rice and chicken";
      let a = "Amala and Ewedu Soup";
      let e = "Eba and Egusi Soup";
      let w = "White rice and Stew";*/
println!("welcome to Mandonald");
println!("
    Our menu:
    Poundo Yam and Edikaiko Soup     #3200
    Fried rice and chicken           #3000
    Amala and Ewedu Soup             #2500
    Eba and Egusi Soup               #2000
    White rice and Stew              #2500

    what may i get for you?
    if Poundo Yam and Edikaiko Soup(press p)
    if Fried rice and chicken (press f)
    if Amala and Ewedu Soup(press a)
    if Eba and Egusi Soup(press e)
    if White rice and Stew(press w)
    ");

let mut cost:i32 = 0;

loop{
    let mut price = String::new();
    println!("so what can i get for you?.....");
    io::stdin().read_line(&mut price).expect("failed to input value");

    if price.trim() == "p"{
        cost+=3200;
    }else if price.trim() == "f"{
        cost +=3200;
    }else if price.trim() == "a"{
        cost +=3200;
    }else if price.trim() == "e"{
        cost +=3200;
    }else if price.trim() == "w"{
        cost +=3200;
    }else{
        break;
    }

}

    if cost >10000{
    println!("for cost more than 1k,you are granted 5% discount");
    let cost:f32 = (cost as f32) * 0.95;
    println!("your price is #{}",cost);
    }else {
        println!("your price is #{}",cost);
    }


}