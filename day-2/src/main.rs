use std::collections::HashMap;
use std::fs::read_to_string;

/// https://adventofcode.com/2022/day/2

fn main() {
    // Read input from file.
    let input = read_to_string("input.txt").unwrap();

    // Function to get the total score for a given strategy.
    let points_for_strategy = |input: &str, truth_table: HashMap<&str, usize>| {
        input
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(|turn| truth_table.get(turn).unwrap())
            .sum::<usize>()
    };

    // Truth table for part 1.
    let truth_table_items = HashMap::from([
        ("A X", 1 + 3), // rock - rock
        ("A Y", 2 + 6), // rock - paper
        ("A Z", 3 + 0), // rock - scissors
        ("B X", 1 + 0), // paper - rock
        ("B Y", 2 + 3), // paper - paper
        ("B Z", 3 + 6), // paper - scissors
        ("C X", 1 + 6), // scissors - rock
        ("C Y", 2 + 0), // scissors - paper
        ("C Z", 3 + 3), // scissors - scissors
    ]);

    // Truth table for part 2.
    let truth_table_results = HashMap::from([
        ("A X", 3 + 0), // rock - scissors
        ("A Y", 1 + 3), // rock - rock
        ("A Z", 2 + 6), // rock - paper
        ("B X", 1 + 0), // paper - rock
        ("B Y", 2 + 3), // paper - paper
        ("B Z", 3 + 6), // paper - scissors
        ("C X", 2 + 0), // scissors - paper
        ("C Y", 3 + 3), // scissors - scissors
        ("C Z", 1 + 6), // scissors - rock
    ]);

    // Print solution of part 1.
    println!(
        "The total amount of points using items would be {}",
        points_for_strategy(&input, truth_table_items)
    );

    // Print solution of part 2.
    println!(
        "The total amount of points using results would be {}",
        points_for_strategy(&input, truth_table_results)
    );
}
