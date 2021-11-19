use std::fs;

fn main() {
    println!("Day 1: Not Quite Lisp");
    println!("Santa wander eratically in a building");

    println!("Tracking Santa moves...");
    let contents = fs::read_to_string("src/day01.txt")
        .expect("Something went wrong reading the file");

    let mut floor = 0;
    let mut has_reached_basement = false;
    for (i, c) in contents.chars().enumerate() {
        floor = if c == '(' {
            floor + 1
        } else if c == ')' {
            floor - 1
        } else {
            floor
        };
        
        if floor < 0 && !has_reached_basement {
            println!("Entering basement for the first time on the {}th move", i+1);
            has_reached_basement = true;
        }
    }

    if !has_reached_basement {
        println!("Santa never reached the basement...");
    }

    println!("The last floor entered is: {}", floor);
}
