use std::io::{self, BufRead};
use std::str;

// Puzzle data type
type Intcode = u32;

// ----------------------------------------------------------------------------

fn main() -> io::Result<()> {
    let program: Vec<Intcode> = io::stdin()
        .lock() // Give access to BufRead::split()
        .split(b',')
        .filter_map(|s| s.ok()) // Discard errors from split()
        .filter_map(|b| {
            // Parse and discard errors
            Intcode::from_str_radix(str::from_utf8(&b).unwrap_or_default(), 10)
                .ok()
        })
        .collect();

    // Part 1
    // ------

    // Tests
    assert_eq!(
        run_program(&mut [1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]),
        [3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
    );
    assert_eq!(run_program(&mut [1, 0, 0, 0, 99]), [2, 0, 0, 0, 99]);
    assert_eq!(run_program(&mut [2, 3, 0, 3, 99]), [2, 3, 0, 6, 99]);
    assert_eq!(
        run_program(&mut [2, 4, 4, 5, 99, 0]),
        [2, 4, 4, 5, 99, 9801]
    );
    assert_eq!(
        run_program(&mut [1, 1, 1, 4, 99, 5, 6, 0, 99]),
        [30, 1, 1, 4, 2, 5, 6, 0, 99]
    );

    // Puzzle answer
    let answer1 = part1(&program);
    println!("Part 1: {}", answer1);
    assert_eq!(answer1, 3716293);

    // Part 2
    // ------

    // Puzzle answer
    let answer2 = part2(&program);
    println!("Part 2: {}", answer2);
    assert_eq!(answer2, 6429);

    Ok(())
}

// ----------------------------------------------------------------------------

fn run_program(mem: &mut [Intcode]) -> &[Intcode] {
    const ADD: Intcode = 1;
    const MUL: Intcode = 2;
    const HALT: Intcode = 99;

    let mut ip = 0; // Instruction Pointer

    let mut r1; // Register 1
    let mut r2; // Register 2
    let mut rr; // Result Register

    loop {
        match mem[ip] {
            ADD => {
                r1 = mem[ip + 1] as usize;
                r2 = mem[ip + 2] as usize;
                rr = mem[ip + 3] as usize;
                mem[rr] = mem[r1] + mem[r2];
                ip += 4;
            }
            MUL => {
                r1 = mem[ip + 1] as usize;
                r2 = mem[ip + 2] as usize;
                rr = mem[ip + 3] as usize;
                mem[rr] = mem[r1] * mem[r2];
                ip += 4;
            }
            HALT => break,
            _ => panic!("BAD OPCODE -- HALT AND CATCH FIRE"),
        }
    }

    mem
}

// ----------------------------------------------------------------------------

fn part1(program: &Vec<Intcode>) -> Intcode {
    let mut mem = program.clone();
    mem[1] = 12;
    mem[2] = 2;
    run_program(&mut mem);

    mem[0]
}

// ----------------------------------------------------------------------------

fn part2(program: &Vec<Intcode>) -> Intcode {
    const TARGET: Intcode = 19690720;

    for noun in 0..100 {
        for verb in 0..100 {
            let mut mem = program.clone();
            mem[1] = noun;
            mem[2] = verb;
            run_program(&mut mem);

            if mem[0] == TARGET {
                return 100 * noun + verb;
            }
        }
    }
    0
}

// ----------------------------------------------------------------------------
