#![feature(iter_array_chunks)]

use std::fs::read_to_string;

/// https://adventofcode.com/2022/day/3

fn main() {
    // Read input from file.
    let input = read_to_string("input.txt").unwrap();

    // Separate the input into individual lines and filter out empty lines.
    let rucksacks = input.split('\n').filter(|line| !line.is_empty());

    // Strategy for part 1.
    let find_item = |[rucksack]: [&str; 1]| {
        let (left, right) = rucksack.split_at(rucksack.len() / 2);
        left.chars().find(|&item| right.contains(item)).unwrap()
    };

    // Strategy for part 2.
    let find_badge = |[rucksack1, rucksack2, rucksack3]: [&str; 3]| {
        rucksack1
            .chars()
            .find(|&item| rucksack2.contains(item) && rucksack3.contains(item))
            .unwrap()
    };

    // Calculate the total priority of all rucksacks with a given strategy.
    fn priority_with_strategy<'i, const N: usize>(
        rucksacks: impl Iterator<Item = &'i str>,
        strategy: impl Fn([&str; N]) -> char,
    ) -> usize {
        // Get the priority for a give item (char).
        let item_to_priority = |item: char| match item {
            'a'..='z' => item as usize - 96,
            'A'..='Z' => item as usize - 38,
            _ => panic!(),
        };

        // Apply strategy to chunks of rucksacks (1 for part one and 3 for part 2).
        rucksacks
            .array_chunks()
            .map(strategy)
            .map(item_to_priority)
            .sum::<usize>()
    }

    // Print result of part 1.
    println!(
        "The total total priority of all items is {}.",
        priority_with_strategy(rucksacks.clone(), find_item)
    );

    // Print result of part 2.
    println!(
        "The total total priority of all badges is {}.",
        priority_with_strategy(rucksacks, find_badge)
    );
}
