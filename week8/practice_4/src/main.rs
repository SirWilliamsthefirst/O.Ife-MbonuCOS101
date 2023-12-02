fn main() {
 //name vector
 let name = vac!["mary","sam","sally","gray","Ada","Mark","june","ife"];


 //age vector
 let age = vec![16,17,19,22,20,18,23];

 println!{"\nAge allocation:\n"};

 //loop to itetrate element in vectors
 for i in 0..age.len(){
    //iterating through i on the vector
    println!("{} is {} years old\n",name[i],age[i]);
 }   
}
