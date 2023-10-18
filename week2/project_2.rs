fn main(){
	let toshiba:f64=450_000.00;
	let mac:f64=1_500_000.00;
	let hp:f64=750_000.00;
	let dell:f64=2_850_000.00;
	let acer:f64=250_000.00;

	//this is frequency for toshiba
	let f1=2.0;

	//this is frequency for mac
	let f2=1.0;

	//this is frequency for hp
	let f3=3.0;

	//this is frequency for dell
	let f4=3.0;

	//this is frequency for acer
	let f5=1.0;

	
	/*to get the sum of sales.
	let sum be x*/

	let x=toshiba+mac+hp+dell+acer;

	println!("The sum of Sales Record is {}",x);
	
	/* to find fx of each item by 
	multipling each item by thier frequency*/

	let fx1:f64=toshiba*f1;

	let fx2:f64=mac*f2;

	let fx3:f64=hp*f3;

	let fx4:f64=dell*f4;

	let fx5:f64=acer*f5;

	//to the the sumation of fx (sfx) 

	let sfx:f64=fx1+fx2+fx3+fx4+fx5;

	//to find the sumation of the freqeuncy(sf)

	let sf:f64=f1+f2+f3+f4+f5;

	//to find the average using the formula average=sfx/sf

	let average:f64=sfx/sf;

	println!("the average is {}",average)

}