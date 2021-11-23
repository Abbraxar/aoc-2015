use std::collections::HashSet;
use std::fs;

struct Position {
    x: i32,
    y: i32
}
impl Position {
    fn left(&mut self) {
        self.x -= 1;
    }

    fn right(&mut self) {
        self.x += 1;
    }

    fn up(&mut self) {
        self.y += 1;
    }

    fn down(&mut self) {
        self.y -= 1;
    }

    fn stay(&self) {}

    fn get_key(&self) -> String {
        format!("{}_{}", self.x, self.y)
    }

    fn next(&mut self, direction: char) {
        match direction {
            '<' => self.left(),
            '>' => self.right(),
            '^' => self.up(),
            'v' => self.down(),
            _ => self.stay(),
        }
    }
}

fn main() {
    println!("Day 3: Perfectly Spherical Houses in a Vacuum");
    println!("Santa wander eratically in the world (not his fault)");

    println!("Tracking Santa moves...");
    let contents = fs::read_to_string("src/day03.txt")
        .expect("Something went wrong reading the file");

    let mut santa_position = Position {
        x: 0,
        y: 0
    };
    let mut robot_santa_position = Position {
        x: 0,
        y: 0
    };
    let mut visited_positions = HashSet::new();
    visited_positions.insert(santa_position.get_key());
    for (i, c) in contents.chars().enumerate() {
        if i % 2 != 0 {
            robot_santa_position.next(c);
            visited_positions.insert(robot_santa_position.get_key());
        } else {
            santa_position.next(c);
            visited_positions.insert(santa_position.get_key());
        }
    }

    println!("Number of houses with one present: {}", visited_positions.len());
}