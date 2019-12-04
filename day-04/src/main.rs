use std::collections::HashMap;

use recap::Recap; // #[recap()]
use serde::Deserialize; // #[derive(Deserialize)]

// ----------------------------------------------------------------------------

// Puzzle data types

#[derive(Deserialize, Recap)]
#[recap(regex = r#"(?P<min>\d+)-(?P<max>\d+)"#)]
struct Input {
    min: i32,
    max: i32,
}

// ----------------------------------------------------------------------------

fn main() -> () {
    let input: Input = "236491-713787".parse().unwrap();

    // Part 1
    // ------

    // Tests
    #[rustfmt::skip]
    {
        assert_eq!(part1(&Input { min: 111111, max: 111111 }), 1);
        assert_eq!(part1(&Input { min: 223450, max: 223450 }), 0);
        assert_eq!(part1(&Input { min: 123789, max: 123789 }), 0);
    }

    // Puzzle answer
    let answer1 = part1(&input);
    println!("Part 1: {}", answer1);
    assert_eq!(answer1, 1169);

    // Part 2
    // ------

    // Tests
    #[rustfmt::skip]
    {
        assert_eq!(part2(&Input { min: 112233, max: 112233 }), 1);
        assert_eq!(part2(&Input { min: 123444, max: 123444 }), 0);
        assert_eq!(part2(&Input { min: 111122, max: 111122 }), 1);

    }

    // Puzzle answer
    let answer2 = part2(&input);
    println!("Part 2: {}", answer2);
    assert_eq!(answer2, 757);
}

// ----------------------------------------------------------------------------

fn check_criteria(number: &str, rep_max: i32) -> bool {
    let mut prev = char::from(('0' as u8) - 1);
    let mut repeated = HashMap::new();

    for c in number.chars() {
        // All digits must be equal or greater than the previous one
        if prev > c {
            return false;
        }

        // Check adjacent digits that are the same
        if prev == c {
            // When a repetition is found, the count is already 2
            *repeated.entry(c).or_insert(1) += 1;
        }

        prev = c;
    }

    // Repeated count must be within the given maximum
    repeated.values().any(|&count| count <= rep_max)
}

// ----------------------------------------------------------------------------

fn part1(input: &Input) -> usize {
    (input.min..=input.max)
        .filter(|n| check_criteria(&n.to_string(), 6))
        .count()
}

// ----------------------------------------------------------------------------

fn part2(input: &Input) -> usize {
    (input.min..=input.max)
        .filter(|n| check_criteria(&n.to_string(), 2))
        .count()
}

// ----------------------------------------------------------------------------
