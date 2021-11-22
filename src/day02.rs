use std::fs;

fn main() {
    println!("Day 2: I Was Told There Would Be No Math");
    println!("Elves are packing big");

    println!("Gathering packaging information...");
    let contents = fs::read_to_string("src/day02.txt")
        .expect("Something went wrong reading the file");

    let mut total_wrapping_surface = 0;
    let mut total_ribbon_length = 0;
    for size_text in contents.split('\n') {
        let sizes: Vec<&str> = size_text.split('x').collect();
        let mut sorted_sizes = [
            &sizes[0].parse().unwrap(),
            &sizes[1].parse().unwrap(),
            &sizes[2].parse().unwrap(),
        ];
        sorted_sizes.sort_unstable();
        let [a, b, c]: [&u32; 3] = sorted_sizes;
        
        total_wrapping_surface += 2 * (a * b + b * c + c * a) + a * b;
        total_ribbon_length += a * b * c + 2 * (a + b);
    }

    println!("Total square feet of wrapping paper: {}", total_wrapping_surface);
    println!("Total ribbon length: {}", total_ribbon_length);
}