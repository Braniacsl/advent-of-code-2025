use std::{fs, ops::RangeInclusive};

type ChristmasRange = RangeInclusive<u64>;

fn parse(contents: String) -> Vec<ChristmasRange> {
    contents
        .trim()
        .split(',')
        .filter_map(|s| {
            // Split into start and end at the dash
            let (start, end) = s.split_once('-')?;
            let start = start.parse::<u64>().ok()?;
            let end = end.parse::<u64>().ok()?;

            Some(start..=end)
        })
        .collect()
}

#[expect(unused)]
fn solve_part1(ranges: Vec<ChristmasRange>) -> u64 {
    let mut grand_total = 0;

    // Check seed lengths from 1 to 9 digits
    for n in 1..=9 {
        // For part 1 the multiplier is just 10^n + 1
        // This generates numbers like 1010 or 1212
        let multiplier = 10u64.pow(n) + 1;

        // Valid seeds must be exactly n digits long
        let seed_min = 10u64.pow(n - 1);
        let seed_max = 10u64.pow(n) - 1;

        for range in &ranges {
            // Find the first and last seed that generate numbers inside the range
            let first_seed = range.start().div_ceil(multiplier);
            let last_seed = range.end() / multiplier;

            // Clamp the seeds to ensure they stick to n digits
            let start_x = first_seed.max(seed_min);
            let end_x = last_seed.min(seed_max);

            if start_x <= end_x {
                // Arithmetic progression sum formula
                let count = end_x - start_x + 1;
                let first_val = start_x * multiplier;
                let last_val = end_x * multiplier;

                grand_total += count * (first_val + last_val) / 2;
            }
        }
    }

    grand_total
}

// calculates the scalar needed to create a repeating pattern
// For length 2 repeated 3 times it calculates 10101
fn get_repeating_multiplier(len: usize, rep_count: usize) -> u64 {
    let mut mult = 0;
    // This is the value we multiply by to shift left by one block
    let block_shift = 10u64.pow(len as u32);
    let mut current_layer: u64 = 1;

    for _ in 0..rep_count {
        // Add the current layer like 1 then 100 then 10000
        mult += current_layer;

        // Safe check to ensure we dont overflow u64 bounds
        if let Some(next) = current_layer.checked_mul(block_shift) {
            current_layer = next;
        }
    }

    mult
}

// Recursively calculates the sum of primitive seeds
// Uses inclusion exclusion principle to subtract seeds formed by smaller periods
fn sum_primitive_seeds(n: usize, min: u64, max: u64) -> u64 {
    // Recursion base case: if the range is invalid return 0
    if min > max {
        return 0;
    }

    // Calculate the raw sum of all numbers in this range
    let count = max - min + 1;
    let mut total_sum = count * (min + max) / 2;

    // Iterate over all divisors of n to find smaller repeating patterns
    for d in 1..n {
        if n.is_multiple_of(d) {
            // Calculate how to scale a seed of length d to length n
            let sub_multiplier = get_repeating_multiplier(d, n / d);

            // Map the current bounds down to the smaller seed space
            let sub_min = min.div_ceil(sub_multiplier);
            let sub_max = max / sub_multiplier;

            // Recursively get the sum of primitive seeds for the divisor
            let sub_sum = sum_primitive_seeds(d, sub_min, sub_max);

            // Subtract these from the total because they look like
            // length n but are actually repeated length d
            total_sum -= sub_sum * sub_multiplier;
        }
    }

    total_sum
}

fn solve_part2(ranges: Vec<ChristmasRange>) -> u64 {
    let mut total = 0;

    // Iterate through all possible seed lengths
    for n in 1..=9 {
        // Iterate through all possible repetition counts
        for k in 2..=20 {
            if n * k > 19 {
                break;
            }

            // Get the multiplier that turns a seed into a full id
            let multiplier = get_repeating_multiplier(n, k);

            // Define the bounds for an n digit seed
            let n_min = 10u64.pow(n as u32 - 1);
            let n_max = 10u64.pow(n as u32) - 1;

            for range in &ranges {
                // Translate the range into seed constraints
                let seed_start = range.start().div_ceil(multiplier);
                let seed_end = range.end() / multiplier;

                // Constrain the seeds to the valid n digit window
                let final_start = seed_start.max(n_min);
                let final_end = seed_end.min(n_max);

                if final_start <= final_end {
                    // Get sum of only the seeds that are length n
                    let prim_sum = sum_primitive_seeds(n, final_start, final_end);

                    // Scale the seed sum back up to the full id values
                    total += prim_sum * multiplier;
                }
            }
        }
    }

    total
}

fn main() {
    let path = std::env::args().nth(1).expect("No path provided");
    let contents = fs::read_to_string(path).expect("Not a valid path");

    let ranges = parse(contents);

    let total = solve_part2(ranges);

    println!("Total is: {}", total);
}
