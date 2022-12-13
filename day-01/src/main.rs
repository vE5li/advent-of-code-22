use std::fs::read_to_string;

/// https://adventofcode.com/2022/day/1

fn main() {
    // Read input from file.
    let input = read_to_string("input.txt").unwrap();

    // Closure to collect all calories from a single elf.
    let per_elf = |elf: &str| {
        elf.split('\n')
            .map(str::parse::<usize>)
            .filter_map(Result::ok)
            .sum::<usize>()
    };

    // Create a vector of total calories that each elf has.
    let mut calories: Vec<usize> = input.split("\n\n").into_iter().map(per_elf).collect();

    // Sort the vector so that the elf with the most calories is at index 0.
    calories.sort_by(|a, b| b.cmp(a));

    // Print the result to Part 1.
    println!(
        "The elf with the most calories is carrying {} calories.",
        calories[0]
    );

    // Print the result to Part 2.
    println!(
        "The top three elfs combined are carrying {} calories.",
        calories[0..3].iter().sum::<usize>()
    );
}
