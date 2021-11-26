use std::fs;

struct Point {
    x: usize,
    y: usize
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }

    fn from(s: &str) -> Point {
        let s: Vec<&str> = s.split(',').collect();
        Point::new(s[0].parse().unwrap(), s[1].parse().unwrap())
    }
}

struct Range {
    p1: Point,
    p2: Point
}

impl Range {
    fn new(p1: Point, p2: Point) -> Range {
        Range { p1, p2 }
    }
}

struct Grid {
    data: [[u32; 1000]; 1000]
}

impl Grid {
    fn new() -> Grid {
        Grid { data: [[0; 1000]; 1000] }
    }

    fn turn(&mut self, state: bool, range: Range) {
        for y in range.p1.y..=range.p2.y {
            for x in range.p1.x..=range.p2.x {
                match state {
                    true => self.data[y][x] += 1,
                    false => match self.data[y][x] {
                        0 => (),
                        _ => self.data[y][x] -= 1
                    },
                }
            }
        }
    }

    fn toggle(&mut self, range: Range) {
        for y in range.p1.y..=range.p2.y {
            for x in range.p1.x..=range.p2.x {
                self.data[y][x] += 2;
            }
        }
    }

    fn brightness(&self) -> u32 {
        let mut brightness = 0;
        for row in self.data.iter() {
            for cell in row.iter() {
                brightness += cell;
            }
        }
        brightness
    }
}

fn main() {
    println!("Day 6: Probably a Fire Hazard");

    println!("Getting instructions...");
    let contents = fs::read_to_string("src/day06.txt")
        .expect("Something went wrong reading the file");
    
    let mut grid = Grid::new();

    for instruction in contents.split('\n') {
        let mut words = instruction.split_whitespace();
        let keyword = words.next().unwrap();
        match keyword {
            "turn" => {
                let state = match words.next().unwrap() {
                    "on" => true,
                    "off" => false,
                    _ => false
                };
                let p1 = Point::from(words.next().unwrap());
                words.next();
                let p2 = Point::from(words.next().unwrap());
                grid.turn(state, Range::new(p1, p2));
            },
            "toggle" => {
                let p1 = Point::from(words.next().unwrap());
                words.next();
                let p2 = Point::from(words.next().unwrap());
                grid.toggle(Range::new(p1, p2));
            },
            _ => continue
        };
    }

    println!("Total brightness: {}", grid.brightness());
}