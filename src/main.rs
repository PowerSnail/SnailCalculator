use std::io;

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).expect("Failed to read line");

    println!("Read {}", input_line);
}
