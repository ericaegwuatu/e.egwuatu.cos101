use std::fs;

fn main() {
	fs::remove_file("data.txt").expect("could not remove file")
    println!("fie is removed");
}
