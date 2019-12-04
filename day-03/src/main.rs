use std::collections::{HashMap, HashSet};
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
        part1(parse_line("R8,U5,L5,D3"), parse_line("U7,R6,D4,L4")),
        6
    );
    assert_eq!(
        part1(
            parse_line("R75,D30,R83,U83,L12,D49,R71,U7,L72"),
            parse_line("U62,R66,U55,R34,D71,R55,D58,R83")
        ),
        159
    );
    assert_eq!(
        part1(
            parse_line("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"),
            parse_line("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")
        ),
        135
    );

    // Puzzle answer
    let answer1 = part1(parse_line(&line1), parse_line(&line2));
    println!("Part 1: {}", answer1);
    assert_eq!(answer1, 896);

    // Part 2
    // ------

    // Tests
    assert_eq!(
        part2(parse_line("R8,U5,L5,D3"), parse_line("U7,R6,D4,L4")),
        30
    );
    assert_eq!(
        part2(
            parse_line("R75,D30,R83,U83,L12,D49,R71,U7,L72"),
            parse_line("U62,R66,U55,R34,D71,R55,D58,R83")
        ),
        610
    );
    assert_eq!(
        part2(
            parse_line("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"),
            parse_line("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")
        ),
        410
    );

    // Puzzle answer
    let answer2 = part2(parse_line(&line1), parse_line(&line2));
    println!("Part 2: {}", answer2);
    assert_eq!(answer2, 16524);

    Ok(())
}

// ----------------------------------------------------------------------------

fn part1(path1: Vec<Path>, path2: Vec<Path>) -> i32 {
    let mut visited = HashSet::new();
    let mut shorter = std::i32::MAX;

    // Store all coordinates visited by path1

    path1.iter().fold(Point2D::new(0, 0), |point, path| {
        let mut new_point = point;

        for _ in 1..=path.len {
            match path.dir {
                'U' => new_point += vec2(0, 1),
                'D' => new_point += vec2(0, -1),
                'L' => new_point += vec2(-1, 0),
                'R' => new_point += vec2(1, 0),
                _ => unreachable!(),
            }
            visited.insert(new_point);
        }
        new_point
    });

    // Check all coordinates visited by path2 and compare when some matches
    // with the coords from path1

    let mut check_cross = |point: &Point2D| {
        if visited.contains(point) {
            // Manhattan distance to the center (0, 0)
            let dist = (point.x - 0).abs() + (point.y - 0).abs();

            if dist < shorter {
                shorter = dist;
            }
        }
    };

    path2.iter().fold(Point2D::new(0, 0), |point, path| {
        let mut new_point = point;

        for _ in 1..=path.len {
            match path.dir {
                'U' => new_point += vec2(0, 1),
                'D' => new_point += vec2(0, -1),
                'L' => new_point += vec2(-1, 0),
                'R' => new_point += vec2(1, 0),
                _ => unreachable!(),
            }
            check_cross(&new_point);
        }
        new_point
    });

    shorter
}

// ----------------------------------------------------------------------------

fn part2(path1: Vec<Path>, path2: Vec<Path>) -> i32 {
    let mut visited = HashMap::new();
    let mut shorter = std::i32::MAX;

    // Store all coordinates visited by path1

    path1
        .iter()
        .fold((Point2D::new(0, 0), 0), |(point, steps), path| {
            let mut new_point = point;
            let mut new_steps = steps;

            for _ in 1..=path.len {
                match path.dir {
                    'U' => new_point += vec2(0, 1),
                    'D' => new_point += vec2(0, -1),
                    'L' => new_point += vec2(-1, 0),
                    'R' => new_point += vec2(1, 0),
                    _ => unreachable!(),
                }
                new_steps += 1;
                visited.entry(new_point).or_insert(new_steps);
            }
            (new_point, new_steps)
        });

    // Find the cross between path1 and path2 with lowest steps

    let mut check_cross =
        |point: &Point2D, steps2: &i32| match visited.get(point) {
            Some(steps1) => {
                let steps_total = steps1 + steps2;

                if steps_total < shorter {
                    shorter = steps_total;
                }
            }
            None => (),
        };

    path2
        .iter()
        .fold((Point2D::new(0, 0), 0), |(point, steps), path| {
            let mut new_point = point;
            let mut new_steps = steps;

            for _ in 1..=path.len {
                match path.dir {
                    'U' => new_point += vec2(0, 1),
                    'D' => new_point += vec2(0, -1),
                    'L' => new_point += vec2(-1, 0),
                    'R' => new_point += vec2(1, 0),
                    _ => unreachable!(),
                }
                new_steps += 1;
                check_cross(&new_point, &new_steps);
            }
            (new_point, new_steps)
        });

    shorter
}

// ----------------------------------------------------------------------------
