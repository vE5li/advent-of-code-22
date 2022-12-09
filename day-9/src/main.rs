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
        let mut chunks = input.split_ascii_whitespace();

        let direction = match chunks.next().unwrap() {
            "U" => Offset { x: 0, y: 1 },
            "D" => Offset { x: 0, y: -1 },
            "L" => Offset { x: -1, y: 0 },
            "R" => Offset { x: 1, y: 0 },
            _ => panic!(),
        };

        let count = chunks.next().unwrap().parse::<usize>().unwrap();

        Self { direction, count }
    }
}

// Function to compute the number of visited position for N number of tail segments.
fn compute_number_of_visited<const N: usize>(input: &str) -> usize {
    // Internal state.
    let mut head_position = Position { x: 0, y: 0 };
    let mut tail_positions = [Position { x: 0, y: 0 }; N];
    // Add the initial tail position to the set.
    let mut visited_positions = HashSet::from([tail_positions[0]]);

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

    // Parse every line as a step.
    let steps = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(Step::parse);

    // Iterate over all steps.
    for step in steps {
        // We always to one step at a time since it makes the computation a lot easier.
        for _ in 0..step.count {
            head_position = head_position + step.direction;

            // Drag all the tails segments behind.
            for index in 0..tail_positions.len() {
                // For the first tail segment we want to use the head position as an anchor, for the rest
                // we use the previous tail segment.
                let anchor_point = match index {
                    0 => head_position,
                    tail => tail_positions[tail - 1],
                };

                // Current position of the tail segment.
                let position = tail_positions[index];
                // Compute the new position of that tail based on it's anchor point (either the
                // head or the previous tail segment).
                let new_position = position + get_tail_move_for_offset(position - anchor_point);
                // Save the new position.
                tail_positions[index] = new_position;
            }

            // Remember the location of the last tail segment.
            visited_positions.insert(tail_positions.last().copied().unwrap());
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
        "The tail with a length of 1 visitied a total of {} different positions.",
        compute_number_of_visited::<1>(&input)
    );

    // Print result of part 2.
    println!(
        "The tail with a length of 9 visitied a total of {} different positions.",
        compute_number_of_visited::<9>(&input)
    );
}
