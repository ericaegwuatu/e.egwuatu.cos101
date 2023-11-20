use std::io;
fn main() {
	let mut input1 = String::new();

    println!("What multiplication table do you want to end ? ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let n:i32 = input1.trim().parse().expect("Not a valid number");

	for i in 1..=n {
    println!("multiplication table (1 to {}):",n );

    for j in 1..=n {
    	println!("{} * {} = {}", i, j, i * j)
	
    }   
  }	 
}
