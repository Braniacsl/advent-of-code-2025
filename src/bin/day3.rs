use std::cmp::max;
use std::io::{self, BufRead};
use std::time::Instant;

// Trait defines shared behavior so we can swap implementations
// Allows us to run different memory strategies without code duplication
trait InvertedIndex {
    fn build(&mut self, bank: &str);
    // Stateful search that remembers its place
    // Finds first valid index for digit within range
    fn find_next(&mut self, digit: usize, min_idx: usize, max_idx: usize) -> Option<usize>;
}

// Naive approach using a vector of vectors
// Simple to write but hurts cache locality due to pointer chasing
#[derive(Default)]
struct NaiveIndex {
    data: [Vec<usize>; 10],
    cursors: [usize; 10], // Tracks current position in each vec
}

impl InvertedIndex for NaiveIndex {
    fn build(&mut self, bank: &str) {
        // Reuse capacity and reset state
        for i in 0..10 {
            self.data[i].clear();
            self.cursors[i] = 0;
        }

        // Standard pass pushing indices into their digit buckets
        bank.bytes().enumerate().for_each(|(i, x)| {
            let x = (x - b'0') as usize;
            self.data[x].push(i);
        });
    }

    fn find_next(&mut self, digit: usize, min_idx: usize, max_idx: usize) -> Option<usize> {
        let vec = &self.data[digit];
        let mut ptr = self.cursors[digit];

        // Scan forward from last known position
        while ptr < vec.len() {
            let val = vec[ptr];
            if val >= min_idx {
                // Found candidate start so save position
                self.cursors[digit] = ptr;

                if val <= max_idx {
                    return Some(val);
                } else {
                    // Vector is sorted so no future values will work
                    return None;
                }
            }
            ptr += 1;
        }

        // Reached end of list
        self.cursors[digit] = ptr;
        None
    }
}

// Optimized approach using a single flat buffer
// Better cache locality and 4x denser memory using u16
#[derive(Default)]
struct FastIndex {
    buffer: Vec<u16>,
    starts: [usize; 11], // 11th slot acts as sentinel
    cursors: [usize; 10],
}

impl InvertedIndex for FastIndex {
    fn build(&mut self, bank: &str) {
        let len = bank.len();
        let bytes = bank.as_bytes();

        // First pass counts occurrences to determine partition sizes
        let mut counts = [0usize; 10];
        for &b in bytes {
            counts[(b - b'0') as usize] += 1;
        }

        // Compute prefix sums to find where each digit partition starts
        let mut current_offset = 0;
        for i in 0..10 {
            self.starts[i] = current_offset;
            self.cursors[i] = current_offset; // Reset cursors to start
            current_offset += counts[i];
        }
        self.starts[10] = current_offset; // Sentinel

        // Reset buffer length without dropping capacity
        self.buffer.clear();
        self.buffer.resize(len, 0);

        // Second pass fills the flat buffer using a mutable cursor
        // This scatters indices into their precalculated slots
        let mut write_pos = self.starts;
        for (i, &b) in bytes.iter().enumerate() {
            let digit = (b - b'0') as usize;
            let pos = write_pos[digit];
            self.buffer[pos] = i as u16;
            write_pos[digit] += 1;
        }
    }

    fn find_next(&mut self, digit: usize, min_idx: usize, max_idx: usize) -> Option<usize> {
        // Use sentinel to avoid storing separate lengths
        let limit = self.starts[digit + 1];
        let mut ptr = self.cursors[digit];

        while ptr < limit {
            // Raw buffer access is cache friendly
            let val = self.buffer[ptr] as usize;

            if val >= min_idx {
                // Found candidate start valid for start condition
                // Update cursor so we never scan previous indices again
                self.cursors[digit] = ptr;

                if val <= max_idx {
                    return Some(val);
                } else {
                    return None;
                }
            }
            ptr += 1;
        }

        self.cursors[digit] = ptr;
        None
    }
}

// Generic solver that accepts any type implementing our trait
// Monomorphization generates two distinct efficient functions
fn solve_p2_generic<T: InvertedIndex + Default>(banks: &[String]) -> u64 {
    let mut indexer = T::default();

    banks
        .iter()
        .map(|bank| {
            indexer.build(bank);

            let mut total = 0;
            let mut cur_pos = 0;
            let len = bank.len();

            for digits_left in (1..=12).rev() {
                let max_valid = len - digits_left;

                // Greedy approach trying largest digits first
                for checking_digit in (0..=9).rev() {
                    // Indexer handles the search logic and state
                    if let Some(idx) = indexer.find_next(checking_digit, cur_pos, max_valid) {
                        total = total * 10 + checking_digit as u64;
                        cur_pos = idx + 1;
                        break;
                    }
                }
            }
            total
        })
        .sum()
}

fn banks_from_file<P>(path: P) -> io::Result<Vec<String>>
where
    P: AsRef<std::path::Path>,
{
    let file = std::fs::File::open(path).expect("invalid path");
    let reader = io::BufReader::new(file);
    reader.lines().collect()
}

// First part only needs to find a pair of digits, rather than 12
fn solve_p1(banks: &[String]) -> usize {
    banks
        .iter()
        .map(|bank| {
            bank.bytes()
                .fold((0, 0), |(max_pair, max_digit), x| {
                    let x = (x - b'0') as usize;
                    let new_pair = max(max_digit * 10 + x, max_pair);
                    let new_x = max(x, max_digit);
                    (new_pair, new_x)
                })
                .0
        })
        .sum()
}

fn main() -> io::Result<()> {
    let start = Instant::now();

    let args: Vec<String> = std::env::args().collect();
    let path = args.get(1).expect("no path given");
    let part: usize = args
        .get(2)
        .expect("no part number")
        .parse()
        .expect("invalid part");
    let optim = args.get(3).map(|s| s == "optim").unwrap_or(false);

    let banks = banks_from_file(path)?;
    let io_time = start.elapsed();

    // Pattern match determines which compiled version runs
    let total = match (part, optim) {
        (1, _) => solve_p1(&banks) as u64,
        (2, false) => solve_p2_generic::<NaiveIndex>(&banks),
        (2, true) => solve_p2_generic::<FastIndex>(&banks),
        _ => panic!("invalid configuration"),
    };

    let total_time = start.elapsed();

    println!("Total is: {}", total);
    println!("Time solving: {:?}", (total_time - io_time));
    println!("Total time: {:?}", total_time);

    Ok(())
}
