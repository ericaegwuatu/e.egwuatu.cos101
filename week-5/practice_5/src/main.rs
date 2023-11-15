use std::io;
fn main() 
{
	let mut input = String::new();

	println!("\nEnter Your Height (in centimeters):");
	io::stdin().read_line(&mut input).expect("Not a valid string");
	let height:f32 = input.trim().parse().expect("Not a valid number");

	if height >= 150.0 && height <= 170.0
	{
		println!("You are of average height");
	}
	else if height > 170.00 && height <= 185.0
	{
		println!("Youe are tall");
	}
	else if height < 150.0 && height >100.0
	{
		println!("You are a drawf");
	}
	else {
		println!("Abnormal height");
    }

}
    
