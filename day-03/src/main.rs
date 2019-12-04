use std::collections::HashMap;
use std::io;

use euclid::*;
use recap::Recap; // #[recap()]
use serde::Deserialize; // #[derive(Deserialize)]

// ----------------------------------------------------------------------------

// Puzzle data types

type Point2D = euclid::default::Point2D<i32>;

#[derive(Debug, Deserialize, Recap)]
#[recap(regex = r#"(?P<dir>\w)(?P<len>\d+)"#)]
struct Path {
    dir: char,
    len: i32,
}

// ----------------------------------------------------------------------------

fn main() -> io::Result<()> {
    fn parse_line(line: &str) -> Vec<Path> {
        line.split(',').filter_map(|s| s.parse().ok()).collect()
    }

    let mut line1 = String::new();
    io::stdin().read_line(&mut line1)?;

    let mut line2 = String::new();
    io::stdin().read_line(&mut line2)?;

    // Part 1
    // ------

    // Tests
    assert_eq!(
        part1(&parse_line("R8,U5,L5,D3"), &parse_line("U7,R6,D4,L4")),
        6
    );
    assert_eq!(
        part1(
            &parse_line("R75,D30,R83,U83,L12,D49,R71,U7,L72"),
            &parse_line("U62,R66,U55,R34,D71,R55,D58,R83")
        ),
        159
    );
    assert_eq!(
        part1(
            &parse_line("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"),
            &parse_line("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")
        ),
        135
    );

    // Puzzle answer
    let answer1 = part1(&parse_line(&line1), &parse_line(&line2));
    println!("Part 1: {}", answer1);
    assert_eq!(answer1, 896);

    // Part 2
    // ------

    // Tests
    assert_eq!(
        part2(&parse_line("R8,U5,L5,D3"), &parse_line("U7,R6,D4,L4")),
        30
    );
    assert_eq!(
        part2(
            &parse_line("R75,D30,R83,U83,L12,D49,R71,U7,L72"),
            &parse_line("U62,R66,U55,R34,D71,R55,D58,R83")
        ),
        610
    );
    assert_eq!(
        part2(
            &parse_line("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"),
            &parse_line("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")
        ),
        410
    );

    // Puzzle answer
    let answer2 = part2(&parse_line(&line1), &parse_line(&line2));
    println!("Part 2: {}", answer2);
    assert_eq!(answer2, 16524);

    Ok(())
}

// ----------------------------------------------------------------------------

fn walk_path(path: &[Path]) -> HashMap<Point2D, i32> {
    let mut walked = HashMap::new();
    let mut point = Point2D::new(0, 0);
    let mut steps = 0;

    for p in path {
        for _ in 1..=p.len {
            match p.dir {
                'U' => point += vec2(0, 1),
                'D' => point += vec2(0, -1),
                'L' => point += vec2(-1, 0),
                'R' => point += vec2(1, 0),
                _ => unreachable!(),
            }
            steps += 1;
            walked.entry(point).or_insert(steps);
        }
    }

    walked
}

// ----------------------------------------------------------------------------

fn part1(path1: &[Path], path2: &[Path]) -> i32 {
    let walk1 = walk_path(path1);
    let walk2 = walk_path(path2);

    walk1
        .keys()
        .filter_map(|point| {
            if walk2.contains_key(point) {
                // Manhattan distance to the center (0, 0)
                Some((point.x - 0).abs() + (point.y - 0).abs())
            } else {
                None
            }
        })
        .min()
        .unwrap_or(0)
}

// ----------------------------------------------------------------------------

fn part2(path1: &[Path], path2: &[Path]) -> i32 {
    let walk1 = walk_path(path1);
    let walk2 = walk_path(path2);

    walk1
        .iter()
        .filter_map(|(point, steps1)| {
            match walk2.get(point) {
                // Sum of steps walked to each cross point
                Some(steps2) => Some(steps1 + steps2),
                None => None,
            }
        })
        .min()
        .unwrap_or(0)
}

// ----------------------------------------------------------------------------
