use std::io;
fn main() 
{
	let mut input1 = String::new();
	let mut input2 = String::new();
	let mut input3 = String::new();

    println!("Enter your name: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");

    println!("Enter your age: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let _age:i32 = input2.trim().parse().expect("Not a valid number");

    println!("Enter your years of experince: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let experince:f32 = input3.trim().parse().expect("Not a valid number");


    if experince >= 40.0 
    {
      println!("Your incentive is N1_560_000.0", input1);
    }
    else if experince >= 30.0 && experince < 40.0 
    {
      println!("Your incentive is N1_480_000.0", input1);
    }
    else if experince <= 28.0 
    {
      println!("Your incentive is N1_300_000.0", input1);
    }
    else if experince <= 0.0 
    {
      println!("Your incentive is N100_000.0", input1 );
    }
 }

