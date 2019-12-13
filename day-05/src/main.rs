use std::cell::Cell;
use std::io;

// ----------------------------------------------------------------------------

// Puzzle data type
type Intcode = i32;

const ADD: Intcode = 1;
const MUL: Intcode = 2;
const IN: Intcode = 3;
const OUT: Intcode = 4;
const JIT: Intcode = 5;
const JIF: Intcode = 6;
const LT: Intcode = 7;
const EQ: Intcode = 8;
const HALT: Intcode = 99;

// ----------------------------------------------------------------------------

fn main() -> io::Result<()> {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("read_line");

    let program: Vec<Intcode> = line
        .trim()
        .split(',')
        .map(|s| s.parse().expect("parse"))
        .collect();

    // Part 1
    // ------

    // Puzzle answer
    let answer1 = part1(&program);
    println!("Part 1: {}", answer1);
    assert_eq!(answer1, 7692125);

    // Part 2
    // ------

    // Puzzle answer
    let answer2 = part2(&program);
    println!("Part 2: {}", answer2);
    assert_eq!(answer2, 14340395);

    Ok(())
}

// ----------------------------------------------------------------------------

fn part1(program: &Vec<Intcode>) -> Intcode {
    let mut mem = program.clone();
    let input = vec![1];

    let output = run_program(&mut mem, &input);

    *output.last().expect("output.last")
}

// ----------------------------------------------------------------------------

fn part2(program: &Vec<Intcode>) -> Intcode {
    let mut mem = program.clone();
    let input = vec![5];

    let output = run_program(&mut mem, &input);

    *output.last().expect("output.last")
}

// ----------------------------------------------------------------------------

fn run_program(mem: &mut [Intcode], input: &[Intcode]) -> Vec<Intcode> {
    let mut input_index = 0;
    let mut output: Vec<Intcode> = Vec::new();

    // Data registers
    let mut r0: Intcode; // Register 1
    let mut r1: Intcode; // Register 2

    // Address register
    let mut _ip = Cell::new(0 as usize); // Instruction Pointer
    let ip = || -> usize {
        let aux = _ip.get();
        _ip.set(_ip.get() + 1);
        aux
    };

    // Remember that write access should never never be in immediate mode
    // #[rustfmt::skip]
    fn mem_access(
        mem: &mut [Intcode],
        i: usize,
        immediate: bool,
    ) -> &mut Intcode {
        if immediate {
            &mut mem[i]
        } else {
            &mut mem[mem[i] as usize]
        }
    }

    fn get_param_modes(incode: Intcode, count: u32) -> Vec<bool> {
        (0..count)
            .map(|i| ((incode / (10_i32.pow(i + 2))) % 10) > 0)
            .collect()
    }

    loop {
        let incode = mem[ip()]; // Whole instruction code
        let opcode = incode % 100;

        match opcode {
            ADD => {
                let pmodes = get_param_modes(incode, 2);
                r0 = *mem_access(mem, ip(), pmodes[0]);
                r1 = *mem_access(mem, ip(), pmodes[1]);
                *mem_access(mem, ip(), false) = r0 + r1;
            }
            MUL => {
                let pmodes = get_param_modes(incode, 2);
                r0 = *mem_access(mem, ip(), pmodes[0]);
                r1 = *mem_access(mem, ip(), pmodes[1]);
                *mem_access(mem, ip(), false) = r0 * r1;
            }
            IN => {
                *mem_access(mem, ip(), false) = input[input_index];
                input_index += 1;
            }
            OUT => {
                let pmodes = get_param_modes(incode, 1);
                r0 = *mem_access(mem, ip(), pmodes[0]);
                output.push(r0);
            }
            JIT => {
                let pmodes = get_param_modes(incode, 2);
                r0 = *mem_access(mem, ip(), pmodes[0]);
                r1 = *mem_access(mem, ip(), pmodes[1]);
                if r0 != 0 {
                    _ip.set(r1 as usize);
                }
            }
            JIF => {
                let pmodes = get_param_modes(incode, 2);
                r0 = *mem_access(mem, ip(), pmodes[0]);
                r1 = *mem_access(mem, ip(), pmodes[1]);
                if r0 == 0 {
                    _ip.set(r1 as usize);
                }
            }
            LT => {
                let pmodes = get_param_modes(incode, 3);
                r0 = *mem_access(mem, ip(), pmodes[0]);
                r1 = *mem_access(mem, ip(), pmodes[1]);
                if r0 < r1 {
                    *mem_access(mem, ip(), false) = 1;
                } else {
                    *mem_access(mem, ip(), false) = 0;
                }
            }
            EQ => {
                let pmodes = get_param_modes(incode, 3);
                r0 = *mem_access(mem, ip(), pmodes[0]);
                r1 = *mem_access(mem, ip(), pmodes[1]);
                if r0 == r1 {
                    *mem_access(mem, ip(), false) = 1;
                } else {
                    *mem_access(mem, ip(), false) = 0;
                }
            }
            HALT => break,
            _ => panic!("BAD OPCODE -- HALT AND CATCH FIRE"),
        }
    }

    output
}

// ----------------------------------------------------------------------------
