use std::io;

// ----------------------------------------------------------------------------

// Puzzle data type
mod intcode;
use intcode::*;

// ----------------------------------------------------------------------------

fn main() -> io::Result<()> {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("read_line");

    // Part 1

    let answer1 = part1(&parse_line(&line));
    println!("Part 1: {}", answer1);
    assert_eq!(answer1, 3280416268);

    // Part 2

    let answer2 = part2(&parse_line(&line));
    println!("Part 2: {}", answer2);
    assert_eq!(answer2, 80210);

    Ok(())
}

// ----------------------------------------------------------------------------

fn parse_line(line: &str) -> Vec<Intcode> {
    line.trim()
        .split(',')
        .map(|s| s.parse().expect("parse"))
        .collect()
}

// ----------------------------------------------------------------------------

fn part1(program: &[Intcode]) -> Intcode {
    let mut process = Process::new(&program);

    process.input.push(1);

    while process.exec() != Status::Halt {}

    if process.output.len() > 1 {
        panic!("Malfunctioning opcodes: {:?}", process.output);
    }

    process.output[0]
}

// ----------------------------------------------------------------------------

fn part2(program: &[Intcode]) -> Intcode {
    let mut process = Process::new(&program);

    process.input.push(2);

    while process.exec() != Status::Halt {}

    process.output[0]
}

// ----------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_quine() {
        let program = parse_line(
            "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99",
        );
        let mut process = Process::new(&program);

        loop {
            let status = process.exec();

            match status {
                Status::WaitForInput => panic!("Unexpected input"),
                Status::NewOutput => (),
                Status::Halt => break,
            }
        }

        assert_eq!(program, process.output);
    }

    #[test]
    fn part1_16digit() {
        let program = parse_line("1102,34915192,34915192,7,4,7,99,0");
        let mut process = Process::new(&program);

        loop {
            let status = process.exec();

            match status {
                Status::WaitForInput => panic!("Unexpected input"),
                Status::NewOutput => (),
                Status::Halt => break,
            }
        }

        assert_eq!(process.output.len(), 1);
        assert_eq!(process.output[0].to_string().chars().count(), 16);
    }

    #[test]
    fn part1_number() {
        let program = parse_line("104,1125899906842624,99");
        let mut process = Process::new(&program);

        loop {
            let status = process.exec();

            match status {
                Status::WaitForInput => panic!("Unexpected input"),
                Status::NewOutput => (),
                Status::Halt => break,
            }
        }

        assert_eq!(process.output.len(), 1);
        assert_eq!(process.output[0], 1125899906842624);
    }
}

// ----------------------------------------------------------------------------
