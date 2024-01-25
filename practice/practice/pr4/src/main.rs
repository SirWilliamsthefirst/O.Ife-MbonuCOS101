fn main() {
    let mut num:i32 = 5;
    mutate_num_to_zero(&mut num);
    println!("this value is {}",num);
}
fn mutate_num_to_zero( par: &mut i32){
    *par = *par * 0;
    println!("par value is :{}",par);
}
