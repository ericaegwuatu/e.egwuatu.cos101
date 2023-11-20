use std::io;
fn main() {
	let mut input1 = String::new();
	let mut input2 = String::new();
	let mut input3 = String::new();
	let mut input4 = String::new();
	let mut input5 = String::new();
	let mut input6 = String::new();
	let mut input7 = String::new();

	
 let mut voters = 0;
 loop {
 	
 	println!("The sytem will only run for the first 150 eligible voters. ");

	println!("Are you a Class Rep ? ");
    io::stdin().read_line(&mut input1).expect("Invalid input. Please answer yes/no");
    let classrep:bool = input1.to_lowercase().trim() == "yes";

    println!("What level are you in ? ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let level:i32 = input2.trim().parse().expect("Not a valid number");

    println!("What is your cgpa ? ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let cgpa:f64 = input3.trim().parse().expect("Not a valid number");


    if level > 100 && cgpa > 4.0 {

    println!("Enter your name : ");
    io::stdin().read_line(&mut input4).expect("Not a valid string");
    
    println!("Enter your email address : ");
    io::stdin().read_line(&mut input5).expect("Not a valid string");

    println!("Enter your department : ");
    io::stdin().read_line(&mut input6).expect("Not a valid string");

    println!("Enter your state of origin : ");
    io::stdin().read_line(&mut input7).expect("Not a valid string");
    
    println!("You can vote!");
    voters +=1;
    println!("{} person(s) have voted",voters);


    } else {
    	println!("Sorry, you are not eligible to vote");
    }
    if voters == 150 {
    	break;
    }
  }
}




