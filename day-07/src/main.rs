use std::io;

use permute;

// ----------------------------------------------------------------------------

// Puzzle data type
mod intcode;
use intcode::*;

// ----------------------------------------------------------------------------

fn main() -> io::Result<()> {
    fn parse_line(line: &str) -> Vec<Intcode> {
        line.trim()
            .split(',')
            .map(|s| s.parse().expect("parse"))
            .collect()
    }

    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("read_line");

    // Part 1
    // ------

    // Tests
    assert_eq!(
        part1(&parse_line(
            "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"
        )),
        43210
    );
    assert_eq!(
        part1(&parse_line("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0")),
        54321
    );
    assert_eq!(
        part1(&parse_line("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0")),
        65210
    );

    // Puzzle answer
    let answer1 = part1(&parse_line(&line));
    println!("Part 1: {}", answer1);
    assert_eq!(answer1, 199988);

    // Part 2
    // ------

    // Tests
    assert_eq!(
        part2(&parse_line("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5")),
        139629729
    );
    assert_eq!(
        part2(&parse_line("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10")),
        18216
    );

    // Puzzle answer
    let answer2 = part2(&parse_line(&line));
    println!("Part 2: {}", answer2);
    assert_eq!(answer2, 17519904);

    Ok(())
}

// ----------------------------------------------------------------------------

fn part1(program: &[Intcode]) -> Intcode {
    permute::permute(vec![0, 1, 2, 3, 4])
        .iter()
        .map(|phases| {
            // Set up

            let mut processes: Vec<_> = phases
                .iter()
                .map(|phase| {
                    let mut process = Process::new(program);
                    process.input.push(*phase as Intcode);
                    process
                })
                .collect();

            // Run

            let mut next_input = 0;

            for (i, _) in phases.iter().enumerate() {
                processes[i].input.push(next_input);
                let status = processes[i].exec();
                assert!(status == Status::NewOutput); // Debug: ensure expected status
                next_input = processes[i].output.pop().expect("No output");
            }
            
            next_input
        })
        .max()
        .unwrap()
}

// ----------------------------------------------------------------------------

fn part2(program: &[Intcode]) -> Intcode {
    permute::permute(vec![5,6,7,8,9])
        .iter()
        .map(|phases| {
            // Set up

            let mut processes: Vec<_> = phases
                .iter()
                .map(|phase| {
                    let mut process = Process::new(program);
                    process.input.push(*phase as Intcode);
                    process
                })
                .collect();

            // Run

            let mut next_input = 0;
            
            for (i, _) in phases.iter().enumerate().cycle() {
                processes[i].input.push(next_input);
                let status = processes[i].exec();
                if status == Status::Halt {
                    break;
                }
                next_input = processes[i].output.pop().expect("No output");
            }

            next_input
        })
        .max()
        .unwrap()
}

// ----------------------------------------------------------------------------
