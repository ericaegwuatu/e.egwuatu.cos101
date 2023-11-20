use std::io;
fn main() {

	let mut input1 = String::new();
	let mut input2 = String::new();


	println!("Enter your distance in miles: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let _distance:f64 = input1.trim().parse().expect("Not a valid number");

    println!("Enter your time in hours : ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let _time:f64 = input2.trim().parse().expect("Not a valid number");

    let km:f64 = _distance * 1.60934;
    println!("The distance in kilometer is {}",km );

    let s:f64 = km / _time;
    println!("The speed is {}",s);

}
