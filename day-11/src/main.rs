use std::collections::HashMap;
use std::io;

use euclid::{point2, Angle};
use turtle::Turtle;

// ----------------------------------------------------------------------------

// Puzzle data types

mod intcode;
use intcode::*;

type Point2D = euclid::default::Point2D<i32>;
type Vector2D = euclid::default::Vector2D<i32>;
type Rotation2D = euclid::default::Rotation2D<f64>;

const BLACK: Intcode = 0;
const WHITE: Intcode = 1;

const LEFT: Intcode = 0;
const RIGHT: Intcode = 1;

// Config

const TURTLE_SIZE: f64 = 5.0;
// const TURTLE_SPEED: i32 = 8;  // Range: [1, 25]
const TURTLE_SPEED: &str = "instant";

// ----------------------------------------------------------------------------

fn main() -> io::Result<()> {
    turtle::start();

    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("read_line");

    // Part 1

    let answer1 = part1(&parse_line(&line));
    println!("Part 1: {}", answer1);
    assert_eq!(answer1, 2883);

    // Part 2

    let answer2 = part2(&parse_line(&line));
    println!("Part 2:\n{}", answer2);
    // assert_eq!(answer2, "LEPCPLGZ");

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

fn part1(program: &[Intcode]) -> usize {
    let painted = paint_ship(program, BLACK, true);

    painted.keys().count()
}

// ----------------------------------------------------------------------------

fn part2(program: &[Intcode]) -> String {
    let painted = paint_ship(program, WHITE, true);

    let min_x = painted.keys().map(|p| p.x).min().unwrap_or(0);
    let max_x = painted.keys().map(|p| p.x).max().unwrap_or(0);
    let min_y = painted.keys().map(|p| p.y).min().unwrap_or(0);
    let max_y = painted.keys().map(|p| p.y).max().unwrap_or(0);

    let mut result = String::new();

    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            let pos = point2(x, y);
            if painted.get(&pos) == Some(&WHITE) {
                result.push('#');
            } else {
                result.push(' ');
            }
        }
        result.push('\n');
    }

    result
}

// ----------------------------------------------------------------------------

fn paint_ship(
    program: &[Intcode],
    start_color: Intcode,
    draw: bool,
) -> HashMap<Point2D, Intcode> {
    let mut process = Process::new(&program);
    let mut visited = HashMap::new();

    // Drawing
    let colors = ["black", "white"];

    let mut turtle = Turtle::new();
    if draw {
        // turtle.drawing_mut().enter_fullscreen();
        turtle.drawing_mut().set_background_color("grey");
        turtle.drawing_mut().set_center((-500.0, 0.0));
        turtle.drawing_mut().set_center((0.0, -180.0));
        turtle.hide();
        turtle.pen_up();
        turtle.set_pen_size(TURTLE_SIZE / 2.0);
        turtle.set_speed("instant");
        turtle.use_degrees();
    }

    // Handling of the current direction
    let to_right = Rotation2D::new(Angle::degrees(-90.0));
    let to_left = Rotation2D::new(Angle::degrees(90.0));

    let mut dir = Vector2D::new(0, 1); // Direction vector
    let mut pos = Point2D::new(0, 0); // Current position
    process.input.push(start_color); // Initial color

    loop {
        // Run the Intcode
        if process.exec() == Status::Halt {
            break;
        }

        if process.output.len() >= 2 {
            let new_color = process.output[0];
            let new_dir = process.output[1];
            process.output.clear();

            // Paint current position
            visited.insert(pos, new_color);

            // Turn
            dir = match new_dir {
                LEFT => to_left.transform_vector(dir.to_f64()).to_i32(),
                RIGHT => to_right.transform_vector(dir.to_f64()).to_i32(),
                _ => panic!("Unexpected direction"),
            };

            // Advance
            pos = pos + dir;

            // Draw
            if draw {
                turtle
                    .set_heading(dir.to_f64().angle_from_x_axis().to_degrees());
                turtle.backward(TURTLE_SIZE / 2.0);

                turtle.set_speed(TURTLE_SPEED);
                turtle.pen_down();
                turtle.set_pen_color(colors[new_color as usize]);
                turtle.forward(TURTLE_SIZE);
                turtle.pen_up();
                turtle.set_speed("instant");

                turtle.forward(TURTLE_SIZE / 2.0);
                turtle.set_speed(TURTLE_SPEED);
            }

            // Add current position's color as next input
            let color = *visited.get(&pos).unwrap_or(&BLACK);
            process.input.push(color);
        }
    }

    visited
}

// ----------------------------------------------------------------------------
