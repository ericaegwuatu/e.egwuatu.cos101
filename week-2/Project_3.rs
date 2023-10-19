fn main() {
	let p:f64 = 210000.0;
	let r:f64 = 5.0;
	let t:f64 = 3.0;

	// amount of the TV after 3 years
	let a = p * (1.0 -(r / 100.0)).powf(t);
	println!("Amount is {}", a);

	// Depriciation
	let d = a - p;
	println!("Depriciation is {}", d);

}