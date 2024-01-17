use std::io::Write;

fn main() {

	let announce = "Week 9 - Rust File Input & Output\n";
	let dept = "Department of Computer Science";

	let mut file = std::fs::file::create("data.txt").expect("create failed");
	file.write_all("Welcome to Rust Programming\n"
		.as_bytes()).expect("write faled");
	file.write_all(announce.as_bytes()).expect("write failed");
	file.write_all(dept.as_byte()).expect("write faied");
    println!("\ndata written to file.");
}
