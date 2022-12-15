#![feature(iter_array_chunks)]
#![feature(iter_next_chunk)]

use std::fs::read_to_string;

// Represents a single step of the rearrangement procedure.
#[derive(Clone, Copy)]
struct Step {
    from: usize,
    to: usize,
    count: usize,
}

impl Step {
    fn parse(step: &str) -> Self {
        let Ok(["move", count, "from", from, "to", to]) = step.split(' ').next_chunk() else {
            panic!("invalid input");
        };

        // Since the stack IDs start at one, we need to subtract one from all indices.
        let count = count.parse::<usize>().unwrap();
        let from = from.parse::<usize>().unwrap() - 1;
        let to = to.parse::<usize>().unwrap() - 1;

        Self { from, to, count }
    }
}

/// https://adventofcode.com/2022/day/5

fn main() {
    // Read input from file.
    let input = read_to_string("input.txt").unwrap();

    // Type alias to make things more readable.
    type Stacks = Vec<Vec<char>>;

    // Split the input into the initial layout of the supply stacks and the opreations we apply to
    // them.
    let (setup, steps) = input.split_once("\n\n").unwrap();

    // Parse the initial layout of the stacks.
    let stacks = {
        // Seperate the setup into seperate lines and *reverse the order*. This way we can just push
        // onto the stack and have the correct order.
        let mut rows = setup.split('\n').rev();

        // Get the first (previously last) line to create the empty stacks. This also has the benefit
        // that we don't need to worry about this line when trying to initialize the stacks down below.
        let mut stacks = vec![Vec::new(); (rows.next().unwrap().len() + 1) / 4];

        // Iterate over all the rows and push the crates onto the stacks.
        for row in rows {
            let mut characters = row.chars();

            for stack in &mut stacks {
                // Try to extract the crate and it's surrounding brackets.
                let Ok([_, crate_id, _]) = characters.next_chunk() else {
                    panic!("invalid input");
                };

                // If there is a crate at this position, add it to the correct stack.
                if crate_id != ' ' {
                    stack.push(crate_id);
                }

                // Remove seperator, will return None on the last stack.
                characters.next();
            }
        }

        stacks
    };

    // Collect and parse all the steps that we will perform later.
    let steps: Vec<Step> = steps
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(Step::parse)
        .collect();

    // Strategy for part 1.
    let move_one_by_one = |stacks: &mut Stacks, step: Step| {
        for _ in 0..step.count {
            let data = stacks[step.from].pop().unwrap();
            stacks[step.to].push(data);
        }
    };

    // Strategy for part 2.
    let move_all_at_once = |stacks: &mut Stacks, step: Step| {
        let insert_index = stacks[step.to].len();
        for _ in 0..step.count {
            let data = stacks[step.from].pop().unwrap();
            stacks[step.to].insert(insert_index, data);
        }
    };

    // Apply a list of steps to our stacks given a certain strategy.
    fn apply_steps_to_stack(
        mut stacks: Stacks,
        steps: &[Step],
        move_function: impl Fn(&mut Stacks, Step),
    ) -> String {
        steps
            .iter()
            .for_each(|step| move_function(&mut stacks, *step));
        stacks.iter().filter_map(|stack| stack.last()).collect()
    }

    // Print results of part 1.
    println!(
        "When moving the crates one at a time, the top of the stack will be {}.",
        apply_steps_to_stack(stacks.clone(), &steps, move_one_by_one)
    );

    // Print results of part 2.
    println!(
        "When moving the crates all at once, the top of the stack will be {}.",
        apply_steps_to_stack(stacks, &steps, move_all_at_once)
    );
}
