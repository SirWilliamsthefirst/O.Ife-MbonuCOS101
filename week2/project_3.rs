fn main(){
	let p:f64=210000.00; // a rep amount
	let r:f64=5.00; // r rep rate
	let n:f64=3.00; // n rep number of years

	//to the principal we have the find r/100 rep by the value x 
	let x:f64=r/100.00;

	// a rep principal
	let a:f64=p*(1.00-x).powf(n);

	//ci rep depreciation
	let ci=p-a;

	println!("The depreciation is #{}",ci);

}