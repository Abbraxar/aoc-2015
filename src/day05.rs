use std::collections::HashMap;
use std::fs;

fn main() {
    println!("Day 5: Doesn't He Have Intern-Elves For This?");
    println!("Santa wonder who has been naughty");

    println!("Spying children...");
    let contents = fs::read_to_string("src/day05.txt")
        .expect("Something went wrong reading the file");

    // OLD SYSTEM
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let forbidden_couples = ["ab", "cd", "pq", "xy"];
    let mut nice_strings_count = 0;
    for phrase in contents.split_whitespace() {
        let mut vowel_count = 0;
        let mut prev = None;
        let mut has_repetition = false;
        let mut has_forbidden_repetition = false;
        for c in phrase.chars() {
            if vowels.contains(&c) {
                vowel_count += 1;
            }
            match prev {
                None => (),
                Some(prevchar) => {
                    if prevchar == c {
                        has_repetition = true;
                    }
                    let couple = format!("{}{}", prevchar, c);
                    if forbidden_couples.contains(&couple.as_str()) {
                        has_forbidden_repetition = true;
                        break;
                    }
                }
            }
            prev = Some(c);
        }
        if !has_forbidden_repetition && has_repetition && vowel_count >= 3 {
            nice_strings_count += 1;
        }
    }

    println!("Old sytem");
    println!("There are {} nice strings.", nice_strings_count);

    // NEW SYSTEM
    let mut nice_strings_count = 0;
    for phrase in contents.split_whitespace() {
        let mut has_repetition = false;
        let mut has_pair_repetition = false;
        let mut pairs = HashMap::new();
        for (i, c) in phrase.chars().enumerate() {
            if i > 1 && phrase.as_bytes()[i - 2] == String::from(c).as_bytes()[0] {
                has_repetition = true;
            }
            if i > 0 {
                let couple = format!("{}{}", phrase.as_bytes()[i - 1], c);
                match pairs.get(&couple) {
                    Some(index) => {
                        if *index < i - 1 {
                            has_pair_repetition = true;
                        }
                    },
                    None => {
                        pairs.insert(couple, i);
                    }
                }
            }
        }
        if has_pair_repetition && has_repetition {
            nice_strings_count += 1;
        }
    }
    println!("New sytem");
    println!("There are {} nice strings.", nice_strings_count);
}