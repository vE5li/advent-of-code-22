use std::fs::read_to_string;
use std::ops::Sub;

// A single instruction that our virtual machine can execute.
#[derive(Debug)]
enum Instruction {
    Add(isize),
    Noop,
}

impl Instruction {
    pub fn parse(line: &str) -> Self {
        match line.split_once(' ') {
            Some(("addx", number)) => Self::Add(number.parse().unwrap()),
            Some(..) => panic!("invalid bytecode"),
            None => Self::Noop,
        }
    }

    // The number of cycles an instruction will take to complete.
    pub fn get_cycle_count(&self) -> usize {
        match self {
            Instruction::Add(..) => 2,
            Instruction::Noop => 1,
        }
    }
}

/// https://adventofcode.com/2022/day/10

fn main() {
    // Read input from file.
    let input = read_to_string("input.txt").unwrap();

    // Constants for part 1.
    const FIRST_MEASUREMENT: isize = 20;
    const REPEAT_MEASUREMENT: isize = 40;

    // Constants for part 2.
    const CRT_WIDTH: usize = 40;
    const CRT_HEIGTH: usize = 40;

    // Mashine state.
    let mut a_register: isize = 1;
    let mut cycle_counter = 1;

    // Variable for part 1.
    let mut total_signal_strength = 0;

    // Variables for part 2.
    let mut horizontal_scan = 0;
    let mut framebuffer = String::with_capacity(CRT_WIDTH * CRT_HEIGTH);

    // Split the input into single lines and parse them as instructions.
    let instructions = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(Instruction::parse);

    // Iterate over all instructions and execute them.
    for instruction in instructions {
        for _ in 0..instruction.get_cycle_count() {
            // Do the measurement.
            if (cycle_counter - FIRST_MEASUREMENT) % REPEAT_MEASUREMENT == 0 {
                let signal_strength = cycle_counter * a_register;
                total_signal_strength += signal_strength;
            }

            // Determine if the pixel is lit or unlit.
            let pixel = match a_register.sub(horizontal_scan as isize).abs() <= 1 {
                true => '#',
                false => '.',
            };

            // Push the pixel to the framebuffer and move the h-scan to the right.
            framebuffer.push(pixel);
            horizontal_scan += 1;

            // Reset the horizontal scan when we hit the end of a line.
            if horizontal_scan == CRT_WIDTH {
                framebuffer.push('\n');
                horizontal_scan = 0;
            }

            // Increment the cycle counter.
            cycle_counter += 1;
        }

        // Modify the register.
        match instruction {
            Instruction::Add(number) => a_register += number,
            Instruction::Noop => {} // Don't to anything.
        }
    }

    // Print result of part 1.
    println!("The total signal strength is {total_signal_strength}.");

    // Print result of part 2.
    println!("CRT output:\n\n{framebuffer}");
}
