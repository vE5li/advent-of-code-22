use std::collections::HashSet;
use std::fs::read_to_string;
use std::ops::{Add, Sub};

// Represents a difference between two positions.
#[derive(Clone, Copy, Debug)]
struct Offset {
    x: isize,
    y: isize,
}

// Represents the position of the head and tail segments.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Position {
    x: isize,
    y: isize,
}

// Subtract two positions, giving us an offset.
impl Sub<Position> for Position {
    type Output = Offset;

    fn sub(self, position: Position) -> Self::Output {
        Offset {
            x: self.x - position.x,
            y: self.y - position.y,
        }
    }
}

// Add an offset to a position, giving us a new position.
impl Add<Offset> for Position {
    type Output = Position;

    fn add(self, offset: Offset) -> Self::Output {
        Position {
            x: self.x + offset.x,
            y: self.y + offset.y,
        }
    }
}

// A single step in our simulation.
#[derive(Clone, Copy, Debug)]
struct Step {
    direction: Offset,
    count: usize,
}

impl Step {
    fn parse(input: &str) -> Self {
        // Split into a character for the direction and the number of steps to take.
        // Example: "U 12" -> "U", "12"
        let (direction, count) = input.split_once(' ').unwrap();

        let direction = match direction {
            "U" => Offset { x: 0, y: 1 },
            "D" => Offset { x: 0, y: -1 },
            "L" => Offset { x: -1, y: 0 },
            "R" => Offset { x: 1, y: 0 },
            _ => panic!(),
        };

        let count = count.parse::<usize>().unwrap();

        Self { direction, count }
    }
}

// Function to compute the number of visited position for N number of tail segments.
fn compute_number_of_visited<const N: usize>(input: &str) -> usize {
    let mut positions = [Position { x: 0, y: 0 }; N];
    // Add the initial tail position to the set.
    let mut visited_positions = HashSet::from([positions[0]]);

    // Parse every line as a step.
    let steps = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(Step::parse);

    // Basically we only care about these 16 cases:
    //
    // TTTTT
    // T...T
    // T.H.T
    // T...T
    // TTTTT
    //
    // Of these 16 cases, the 4 cases where the tail segment is in the corner will pull the segment diagonally:
    //
    // .....
    // .T.T.
    // ..H..
    // .T.T.
    // .....
    //
    // And since the remaining 12 cases will always be pulled to the side of the head, they can be boiled down
    // to 4 different outcomes:
    //
    // .....
    // ..T..
    // .THT.
    // ..T..
    // .....
    let get_tail_move_for_offset = |offset: Offset| match offset {
        // 4 special cases:
        Offset { x: -2, y: -2 } => Offset { x: 1, y: 1 },
        Offset { x: -2, y: 2 } => Offset { x: 1, y: -1 },
        Offset { x: 2, y: -2 } => Offset { x: -1, y: 1 },
        Offset { x: 2, y: 2 } => Offset { x: -1, y: -1 },
        // 12 regular cases:
        Offset { x: -2, y } => Offset { x: 1, y: -y },
        Offset { x: 2, y } => Offset { x: -1, y: -y },
        Offset { x, y: 2 } => Offset { x: -x, y: -1 },
        Offset { x, y: -2 } => Offset { x: -x, y: 1 },
        _ => Offset { x: 0, y: 0 },
    };

    // Iterate over all steps.
    for step in steps {
        // We always to one step at a time since it makes the computation a lot easier.
        for _ in 0..step.count {
            positions[0] = positions[0] + step.direction;

            // Drag all the tails segments behind.
            for index in 1..positions.len() {
                // Position of the previous segment.
                let anchor_point = positions[index - 1];
                // Current position of the tail segment.
                let position = positions[index];

                // Compute the new position of that tail based on it's anchor point.
                let new_position = position + get_tail_move_for_offset(position - anchor_point);
                // Save the new position.
                positions[index] = new_position;
            }

            // Remember the location of the last tail segment.
            visited_positions.insert(positions.last().copied().unwrap());
        }
    }

    // Return the total number of locations visited by the last tail segment.
    visited_positions.len()
}

/// https://adventofcode.com/2022/day/9

fn main() {
    // Read input from file.
    let input = read_to_string("input.txt").unwrap();

    // Print result of part 1.
    println!(
        "The rope with a length of 2 visitied a total of {} different positions.",
        compute_number_of_visited::<2>(&input)
    );

    // Print result of part 2.
    println!(
        "The rope with a length of 10 visitied a total of {} different positions.",
        compute_number_of_visited::<10>(&input)
    );
}
