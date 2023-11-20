use std::io;

fn main() {
	let mut input1 = String::new();
	let mut input2 = String::new();
	let mut input3 = String::new();
	let mut input4 = String::new();
	let mut input5 = String::new();
	let mut input6 = String::new();
	let mut input7 = String::new();
	let mut input8 = String::new();
	let mut input9 = String::new();

	
	let r1 = alzheimer;
	let a1 = arrythmia;
	let c1 = Chronic_kidney_disease;
	let d1 = diabeties;
	let s1 = arthritis;

	let k2 = akapabom;
	let n2 = ngbauji;
	let a2 = atabrikang;
	let o2 = okorobilom;
	let e2 = emeremen;



println!("Enter your name:");
	io::stdin().read_line(&mut input1).except("Not a valid string");

	println!("Enter your age:");
	io::stdin().read_line(&mut input2).except("Not a valid string");
	let a:i32 = input2.trim().parse().expect("Not a valid number");

	println!("Enter your email address:");
	io::stdin().read_line(&mut input3).except("Not a valid string");
	let e:i32 = input3.trim().parse().expect("Not a valid number");

	println!("Enter your phone number:");
	io::stdin().read_line(&mut input4).except("Not a valid string");
	let p:i32 = input4.trim().parse().expect("Not a valid number");

	println!("Enter your number of siblings:");
	io::stdin().read_line(&mut input5).except("Not a valid string");
	let s:i32 = input5.trim().parse().expect("Not a valid number");

	println!("Enter your number of children:");
	io::stdin().read_line(&mut input6).except("Not a valid string");
	let c:i32 = input6.trim().parse().expect("Not a valid number");

	println!("Enter your medical daignosis:");
	io::stdin().read_line(&mut input7).except("Not a valid string");
	let m:i32 = input7.trim().parse().expect("Not a valid number");

	println!("Enter your village of residence:");
	io::stdin().read_line(&mut input8).except("Not a valid string");
	let v:i32 = input8.trim().parse().expect("Not a valid number");

	println!("Enter your user number:");
	io::stdin().read_line(&mut input9).except("Not a valid string");
	let u:i32 = input9.trim().parse().expect("Not a valid number");

loop {	
if d = r1 && a > 50.0 && c > 4.0 && v = k2
	{
	   let amount1:f32 = 1200000 * 0.8;
	   println!("Your name is {}", input1);
       println!("Your age is {}", input2);
	   println!("Your email address is {}", input3);
       println!("Your phone number is {}", input4);
	   println!("Your number of siblings is {}", input5);
       println!("Your number of children is {}", input6);
       println!("Your medical daignosis is {}", input7);
	   println!("Your village of residence is {}", input8);
	   println!("Your calculated amount is N{}", amount1)

	}else {
		println!("The normal chargoe of N1200000 applies");
	}

if d = a1 && a = 30.0 && s > 4.0 && v = n2
	{
	   let amount2:f32 = 550000 * 0.95;
	   println!("Your name is {}", input1);
       println!("Your age is {}", input2);
	   println!("Your email address is {}", input3);
       println!("Your phone number is {}", input4);
	   println!("Your number of siblings is {}", input5);
       println!("Your number of children is {}", input6);
       println!("Your medical daignosis is {}", input7);
	   println!("Your village of residence is {}", input8);
	   println!("Your calculated amount is N{}", amount2)

	}else {
		println!("The normal chargoe of N550000 applies");
	}

    
if d = c1 && a > 40.0 && c > 3.0 && s > 3.0 && v = a2
	{
	   let amount3:f32 = 1550000 * 0.85;
	   println!("Your name is {}", input1);
       println!("Your age is {}", input2);
	   println!("Your email address is {}", input3);
       println!("Your phone number is {}", input4);
	   println!("Your number of siblings is {}", input5);
       println!("Your number of children is {}", input6);
       println!("Your medical daignosis is {}", input7);
	   println!("Your village of residence is {}", input8);
	   println!("Your calculated amount is N{}", amount3)

	}else {
		println!("The normal chargoe of N1550000 applies");
	}


if d = d1 && a > 28.0 && a < 45.0 && c >= 2.0 && c <= 4.0 && v = o2

	{
	   let amount3:f32 = 800000 * 0.90;
	   println!("Your name is {}", input1);
       println!("Your age is {}", input2);
	   println!("Your email address is {}", input3);
       println!("Your phone number is {}", input4);
	   println!("Your number of siblings is {}", input5);
       println!("Your number of children is {}", input6);
       println!("Your medical daignosis is {}", input7);
	   println!("Your village of residence is {}", input8);
	   println!("Your calculated amount is N{}", amount3)

	}else {
		println!("The normal charges of N800000 applies");
	}


if d = s1 && a > 58.0 && c > 5.0 && s > 5.0 && v = e2
	{
	   let amount3:f32 = 450000 * 0.90;
	   println!("Your name is {}", input1);
       println!("Your age is {}", input2);
	   println!("Your email address is {}", input3);
       println!("Your phone number is {}", input4);
	   println!("Your number of siblings is {}", input5);
       println!("Your number of children is {}", input6);
       println!("Your medical daignosis is {}", input7);
	   println!("Your village of residence is {}", input8);
	   println!("Your calculated amount is N{}", amount3)

	}else {
		println!("The normal charges of N450000 applies");
	}

if u == 20{
	break;
}
}
}
