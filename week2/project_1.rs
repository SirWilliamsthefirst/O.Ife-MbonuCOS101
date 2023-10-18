 fn main() {
	let p:f64=520000000.0;//let p be the profit
	let r:f64=10.0;//let r be the rate
	let n:f64=5.0;//let n be the number of years
	
	/*to solve the problem with the formula A=P[1 + (r/100)^n]
	let 1+(r/100) represent the value x*/

	let x:f64=1.0+(r/100.0);

	let a:f64=p*(x.powf(n));

	let ci =a-p;

	println!("The compound interest is {}",ci);
}