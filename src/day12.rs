use std::fs::read_to_string;

struct Part1 {
    position: (i32, i32),
    facing: i32,
}

impl Part1 {
    pub fn next(&mut self, instruction: char, delta: i32) {
        match instruction {
            'N' => self.position.1 -= delta,
            'S' => self.position.1 += delta,
            'E' => self.position.0 += delta,
            'W' => self.position.0 -= delta,
            'R' => self.facing = (self.facing + delta) % 360,
            'L' => self.facing = (self.facing - delta + 360) % 360,
            'F' => match self.facing {
                0 => self.position.1 -= delta,
                90 => self.position.0 += delta,
                180 => self.position.1 += delta,
                270 => self.position.0 -= delta,
                _ => panic!(),
            },
            _ => panic!(),
        }
    }
}

struct Part2 {
    position: (i32, i32),
    waypoint: (i32, i32),
}

impl Part2 {
    fn rot(waypoint: (i32, i32), mut delta: i32) -> (i32, i32) {
        delta = (delta % 360 + 360) % 360;

        match delta {
            0 => waypoint,
            90 => (-waypoint.1, waypoint.0),
            180 => (-waypoint.0, -waypoint.1),
            270 => (waypoint.1, -waypoint.0),
            _ => {
                panic!();
            }
        }
    }

    pub fn next(&mut self, instruction: char, delta: i32) {
        match instruction {
            'N' => self.waypoint.1 -= delta,
            'S' => self.waypoint.1 += delta,
            'E' => self.waypoint.0 += delta,
            'W' => self.waypoint.0 -= delta,
            'R' => self.waypoint = Self::rot(self.waypoint, delta),
            'L' => self.waypoint = Self::rot(self.waypoint, -delta),
            'F' => {
                self.position.0 += self.waypoint.0 * delta;
                self.position.1 += self.waypoint.1 * delta;
            }
            _ => panic!(),
        }
    }
}

pub fn main() {
    let text = read_to_string("inputs/day12.in").unwrap();

    let instructions = text.trim().split("\n").map(|line| {
        (
            line.chars().next().unwrap(),
            i32::from_str_radix(&line[1..], 10).unwrap(),
        )
    });

    let mut p1 = Part1 {
        position: (0, 0),
        facing: 90,
    };

    let mut p2 = Part2 {
        position: (0, 0),
        waypoint: (10, -1),
    };

    for (direction, delta) in instructions {
        p1.next(direction, delta);
        p2.next(direction, delta);
    }

    println!("{}", p1.position.0.abs() + p1.position.1.abs());
    println!("{}", p2.position.0.abs() + p1.position.1.abs());
}
