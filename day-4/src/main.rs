use std::fs::read_to_string;
use std::num::ParseIntError;
use std::ops::RangeInclusive;

/// https://adventofcode.com/2022/day/4

fn main() {
    // Read input from file.
    let input = read_to_string("input.txt").unwrap();

    // Type alias to make things a bit more readable.
    type Range = RangeInclusive<usize>;

    // Strategy for part 1.
    let overlap_completely = |range1: Range, range2: Range| {
        range1.contains(range2.start()) && range1.contains(range2.end())
            || range2.contains(range1.start()) && range2.contains(range1.end())
    };

    // Strategy for part 2.
    let overlap_partially = |range1: Range, range2: Range| {
        range1.end() >= range2.start() && range1.start() <= range2.end()
            || range2.end() >= range1.start() && range2.start() <= range1.end()
    };

    // Count the number of ranges that overlap for a given strategy.
    fn count_for_strategy(input: &str, strategy: impl Fn(Range, Range) -> bool) -> usize {
        // Attempt to parse a string of format X-X as a range.
        let parse_range = |range: &str| -> Result<Range, ParseIntError> {
            let (from, to) = range.split_once('-').unwrap();
            Ok(from.parse()?..=to.parse()?)
        };

        // Apply strategy for a given line.
        let apply_strategy = |line: &str| {
            let (left, right) = line.split_once(',').unwrap();
            let range1 = parse_range(left).unwrap();
            let range2 = parse_range(right).unwrap();
            strategy(range1, range2)
        };

        // Iterate over the input and apply our strategy, then count the number of lines that
        // matched:
        input
            .split('\n')
            .filter(|line| !line.is_empty())
            .filter(|range| apply_strategy(range))
            .count()
    }

    // Print results of part 1.
    println!(
        "A total of {} ranges are overlapping completely.",
        count_for_strategy(&input, overlap_completely)
    );

    // Print results of part 2.
    println!(
        "A total of {} ranges are overlapping partially.",
        count_for_strategy(&input, overlap_partially)
    );
}
