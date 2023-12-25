use sha256::digest;
use std::io;
use std::io::prelude::*;

fn main() {

    println!("Enter a value to hash: ");
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let input = String::from(line.unwrap());
        let val = digest(input);
        println!("Hashed Value: {}", val);
    }

}
