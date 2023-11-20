use std::io;

fn main() {
	let mut input1 = String::new();
	let mut input2 = String::new();

let mut researchers = 0;
 loop {

	println!("The RPIS will be executed for only 500 researchers. ");
    
    println!("What is the name of your Researchers Publication ? ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    
    println!("What is the number of paper you have published? ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let number:i32 = input2.trim().parse().expect("Not a valid number");

  if number >= 3 && number <= 5 {
  println!("{}",input1 );
  println!("Your incentive is N500000");
  } 

  if number > 5 && number < 10 {
  println!("{}",input1 );
  println!("Your incentive is N800000");
  }

  if number > 10 {
  println!("{}",input1 );
  println!("Your incentive is N1000000");
  }

  if number < 3 {
  println!("{}",input1 );
  println!("Your incentive is N800000");
  }

  researchers +=1;
  println!("The system has exacuted for {} researchers",researchers);

  if researchers == 150 {
    	break;
    }
}
}

