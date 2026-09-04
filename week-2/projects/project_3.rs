fn main () {
	let p:f64 = 210_000.0;
	let r:f64 = 5.0;
	let n:f64 = 3.0;

	let depreciated_value:f64 = p * (1.0-(r/100.0)).powf(n);
	
	println! ("the value of the Tv fter three years is {} ", depreciated_value);
}