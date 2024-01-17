fn main() {

	let v =vec!['C','O','M','P','U','T','E','R'];

	let mut input1 = String::new();

	println!("Enter an index value btw (0 - 7)");
	std::10::stdin().read_line(&mut input1).expect("Failed to read input");
	let index:usize = input1.trim().parse().expect("Invalid input");
    println!("Hello, world!");
}