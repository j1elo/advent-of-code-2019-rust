use std::collections::HashMap;
use std::io;

use console::{style, Term};

// ----------------------------------------------------------------------------

// Puzzle data types

mod intcode;
use intcode::*;

const EMPTY: Intcode = 0;
const WALL: Intcode = 1;
const BLOCK: Intcode = 2;
const PADDLE: Intcode = 3;
const BALL: Intcode = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point2D {
    x: Intcode,
    y: Intcode,
}

// Config

//const SPEED: u64 = 0; // FAST, good for quick puzzle answer
const SPEED: u64 = 40; // SLOW, good for visualization

// ----------------------------------------------------------------------------

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("read_line");

    // Part 1

    let answer1 = part1(&parse_line(&line));
    println!("Part 1: {}", answer1);
    assert_eq!(answer1, 326);

    // Part 2

    let answer2 = part2(&parse_line(&line)).expect("part2");
    println!("Part 2: {}", answer2);
    assert_eq!(answer2, 15988);
}

// ----------------------------------------------------------------------------

fn parse_line(line: &str) -> Vec<Intcode> {
    line.trim()
        .split(',')
        .map(|s| s.parse().expect("parse"))
        .collect()
}

// ----------------------------------------------------------------------------

fn part1(program: &[Intcode]) -> usize {
    let mut process = Process::new(&program);
    let mut tiles = HashMap::new();

    loop {
        // Run the Intcode
        if process.exec() == Status::Halt {
            break;
        }

        if process.output.len() >= 3 {
            // Interpret output values
            let pos = (process.output[0], process.output[1]);
            let id = process.output[2];
            process.output.clear();

            // Paint current position
            tiles.insert(pos, id);
        }
    }

    tiles.values().filter(|id| id == &&BLOCK).count()
}

// ----------------------------------------------------------------------------

fn part2(program: &[Intcode]) -> io::Result<Intcode> {
    let mut program = program.to_vec();
    // Hack the "Insert Coin" screen!
    program[0] = 2;
    let mut process = Process::new(&program);

    let mut score = 0;
    let mut ball_pos = Point2D { x: 0, y: 0 };
    let mut paddle_pos = Point2D { x: 0, y: 0 };

    let term = Term::stdout();
    term.hide_cursor()?;
    term.clear_screen()?;

    loop {
        // Run the Intcode
        let status = process.exec();

        if status == Status::Halt {
            break;
        } else if status == Status::WantInput {
            if ball_pos.x > paddle_pos.x {
                process.input.push(1)
            } else if ball_pos.x < paddle_pos.x {
                process.input.push(-1)
            } else {
                process.input.push(0)
            }
            std::thread::sleep(std::time::Duration::from_millis(SPEED));
        } else if process.output.len() >= 3 {
            // Interpret output values
            let pos = Point2D {
                x: process.output[0],
                y: process.output[1],
            };

            if pos.x == -1 && pos.y == 0 {
                // Score mode
                score = process.output[2];

                term.move_cursor_to(0, 0)?;
                term.write_str(&format!("score: {}", score))?;
            } else {
                // Tile coordinate mode
                let id = process.output[2];
                let c = match id {
                    // EMPTY => ' ',
                    // WALL => '|',
                    // BLOCK => '#',
                    // PADDLE => '=',
                    // BALL => '*',
                    EMPTY => style(' '),
                    WALL => style(' ').on_white(),
                    BLOCK => style(' ').on_yellow(),
                    PADDLE => style('=').green(),
                    BALL => style('@').red(),
                    _ => panic!("Unexpected tile id"),
                };

                if id == BALL {
                    ball_pos = pos;
                } else if id == PADDLE {
                    paddle_pos = pos;
                }

                term.move_cursor_to(pos.x as usize, pos.y as usize)?;
                term.write_str(&format!("{}", c))?;
            }

            process.output.clear();
        }
    }

    term.clear_screen()?;

    Ok(score)
}

// ----------------------------------------------------------------------------
