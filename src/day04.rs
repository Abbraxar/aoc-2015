use md5;
use std::fs;

fn main() {
    println!("Day 4: The Ideal Stocking Stuffer");
    println!("Santa is a crypto bro");

    println!("Fetching secret key...");
    let secret_key = fs::read_to_string("src/day04.txt")
        .expect("Something went wrong reading the file");

    let mut counter = 0;
    loop {
        counter = counter + 1;
        let to_hash = format!("{}{}", secret_key, counter);
        let digest = format!("{:x}", md5::compute(to_hash));
        if digest.starts_with("000000") {
            break;
        }
    };
    println!("Mined: {}", counter);
}