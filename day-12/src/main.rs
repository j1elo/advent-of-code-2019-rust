// use std::io::{self, BufRead};

// use recap::{from_captures, Regex};

// // ----------------------------------------------------------------------------

// // Puzzle data types

// type Point3D = euclid::default::Point3D<i32>;

// // ----------------------------------------------------------------------------

// fn main() {
//     let points: Vec<Point3D> = io::stdin()
//         .lock() // Give access to BufRead::lines()
//         .lines()
//         .map(|l| l.expect("lines"))
//         .map(|l| parse_line(&l).expect("parse_line"))
//         .collect();

//     // Part 1

//     let answer1 = part1(&points);
//     println!("Part 1: {}", answer1);
//     assert_eq!(answer1, 0);

//     // Part 2

//     let answer2 = part2(&points);
//     println!("Part 2: {}", answer2);
//     assert_eq!(answer2, 0);
// }

// // ----------------------------------------------------------------------------

// // fn parse_line(line: &str) -> Option<Point3D> {
// //     Regex::new(r#"<x=(?P<x>-?\d+), y=(?P<y>-?\d+), z=(?P<z>-?\d+)>"#)
// //         .ok()
// //         .and_then(|pattern| from_captures(&pattern, line).ok())
// // }

// fn parse_line(_line: &str) -> Option<Point3D> {
//     // let regex = Regex::new(r#"^<x=(?P<x>-?\d+), y=(?P<y>-?\d+), z=(?P<z>-?\d+)>(?P<_unit>.*)$"#).expect("AAA");
//     // let regex = Regex::new(r#"A=(?P<x>\d+), B=(?P<y>\d+), C=(?P<z>\d+)"#).expect("check 1");
//     let regex = Regex::new(r#"A=(?P<x>\d+), B=(?P<y>\d+), C=(?P<z>\d+)(?P<_unit>.*)"#).expect("check 1");
//     let line = "A=1, B=2, C=3";
//     let point = from_captures::<Point3D>(&regex, line).expect("check 2");
//     Some(point)
// }

// // ----------------------------------------------------------------------------

// fn part1(_points: &[Point3D]) -> usize {
//     0
// }

// // ----------------------------------------------------------------------------

// fn part2(_points: &[Point3D]) -> usize {
//     0
// }

// ---------------------------------------------------------------------------------------------------------------------

use recap::{from_captures, Regex};
type Point3D = euclid::default::Point3D<i32>;

fn main() {
    let line = "A=1, B=2, C=3";
    
    let regex = Regex::new(r#"A=(?P<x>\d+), B=(?P<y>\d+), C=(?P<z>\d+)"#).expect("check 1");
    // let regex = Regex::new(r#"A=(?P<x>\d+), B=(?P<y>\d+), C=(?P<z>\d+)(?P<_unit>.*)"#).expect("check 1");
    let _point: Point3D = from_captures::<Point3D>(&regex, line).expect("check 2");
}
