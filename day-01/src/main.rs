use std::io::{self, BufRead};

// Puzzle data type
type Mass = i32;
type Fuel = i32;

// ----------------------------------------------------------------------------

fn main() -> io::Result<()> {
    let masses: Vec<Mass> = io::stdin()
        .lock() // Give access to BufRead::lines()
        .lines()
        .filter_map(|l| l.ok()) // Discard errors from lines()
        .filter_map(|l| l.parse().ok()) // Parse and discard errors
        .collect();

    // Part 1
    // ------

    // Tests
    assert_eq!(part1(&[12, 14, 1969, 100756]), 34241);

    // Puzzle answer
    let answer1 = part1(&masses);
    println!("Part 1: {}", answer1);
    assert_eq!(answer1, 3232358);

    // Part 2
    // ------

    // Tests
    assert_eq!(part2(&[14, 1969, 100756]), 51314);

    // Puzzle answer
    let answer2 = part2(&masses);
    println!("Part 2: {}", answer2);
    assert_eq!(answer2, 4845669);

    Ok(())
}

// ----------------------------------------------------------------------------

fn part1(masses: &[Mass]) -> Fuel {
    masses.iter().map(|mass| (mass / 3) - 2).sum()
}

// ----------------------------------------------------------------------------

fn part2(masses: &[Mass]) -> Fuel {
    masses.iter().map(|mass| calc_fuel(*mass)).sum()
}

fn calc_fuel(mass: Mass) -> Fuel {
    let fuel = (mass / 3) - 2;
    if fuel > 0 {
        fuel + calc_fuel(fuel)
    } else {
        0
    }
}

// ----------------------------------------------------------------------------
