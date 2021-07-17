use std::io::{self, BufRead};
use recap::Recap;
use serde::Deserialize;

// ----------------------------------------------------------------------------

// Puzzle data types

#[derive(Debug, Deserialize, Recap)]
#[recap(regex = r#"(?P<quantity>\d+) (?P<name>\w+)"#)]
struct Chemical {
    quantity: i32,
    name: String,
}

#[derive(Debug, Deserialize, Recap)]
#[recap(regex = r#"^(?P<inputs>(\d+ \w+(, )?)+) => (?P<output>(\d+ \w+))$"#)]
// #[recap(regex = r#"^(?P<inputs>.*) => (?P<output>(\d+ \w+))$"#)]
// #[recap(regex = r#"(?P<inputs>((?P<quantity>\d+) (?P<name>\w+)(, )?)+) => (?P<output>(\d+ \w+))"#)]
struct Reaction {
    inputs: Vec<Chemical>,
    output: Chemical,
}

// ----------------------------------------------------------------------------

fn main() {
    let reactions: Vec<Reaction> = io::stdin()
        .lock() // Give access to BufRead::lines()
        .lines()
        .map(|line_result| line_result.expect("stdin.lines"))
        .map(|line| line.parse().expect("line.parse"))
        .collect();

    println!("reactions: {:?}", reactions);

    // Part 1

    // let answer1 = part1(&points);
    // println!("Part 1: {}", answer1);
    // assert_eq!(answer1, 0);

    // Part 2

    // let answer2 = part2(&points);
    // println!("Part 2: {}", answer2);
    // assert_eq!(answer2, 0);
}

// ----------------------------------------------------------------------------

// fn part1(_points: &[Point3D]) -> usize {
//     0
// }

// ----------------------------------------------------------------------------

// fn part2(_points: &[Point3D]) -> usize {
//     0
// }

// ---------------------------------------------------------------------------------------------------------------------
