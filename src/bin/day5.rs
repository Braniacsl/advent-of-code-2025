use advent_of_code_2025::aoc_main;
use std::sync::Arc;
use std::thread;

type ChristmasRange = std::ops::RangeInclusive<u64>;

fn create_merged_sorted_ranges(elf_db: &str) -> Vec<ChristmasRange> {
    // Create Vec of ranges (normal parse)
    let mut ranges: Vec<ChristmasRange> = elf_db
        .lines()
        .map(|l| {
            let (start, end) = l.split_once('-').unwrap();
            start.parse().unwrap()..=end.parse().unwrap()
        })
        .collect();

    // Sort vec by range start for easier merge & later logic
    ranges.sort_by(|a, b| a.start().cmp(b.start()));

    // Merge sorted ranges by folding. Accumulator keeps our final result so either merge and add
    // or just add to acc
    let mut iter = ranges.into_iter();
    let first = iter.next().unwrap();

    iter.fold(vec![first], |mut acc, r| {
        let last = acc.last_mut().unwrap();

        if *r.start() <= *last.end() + 1 {
            let new_end = std::cmp::max(*last.end(), *r.end());
            *last = *last.start()..=new_end;
        } else {
            acc.push(r);
        }
        acc
    })
}

fn solve_p1(elf_db: &str) -> u64 {
    let (ranges, ids) = elf_db.split_once("\n\n").unwrap();
    let ranges = Arc::new(create_merged_sorted_ranges(ranges));
    let ids: Arc<Vec<u64>> = Arc::new(ids.trim().lines().map(|s| s.parse().unwrap()).collect());

    // Calculate ids (in chunk) per thread
    let num_threads = thread::available_parallelism().unwrap().get();
    let chunk_size = ids.len().div_ceil(num_threads);

    let handles: Vec<_> = (0..num_threads)
        .map(|i| {
            let ranges = ranges.clone();
            let ids = ids.clone();

            thread::spawn(move || {
                let start = i * chunk_size;
                let end = std::cmp::min((i + 1) * chunk_size, ids.len());
                if start >= ids.len() {
                    return 0;
                }
                let chunk = &ids[start..end];

                chunk
                    .iter()
                    .filter(|id| {
                        ranges
                            .binary_search_by(|range| {
                                if range.contains(id) {
                                    std::cmp::Ordering::Equal
                                } else if *range.start() > **id {
                                    std::cmp::Ordering::Greater
                                } else {
                                    std::cmp::Ordering::Less
                                }
                            })
                            .is_ok()
                    })
                    .count()
            })
        })
        .collect();

    handles.into_iter().map(|h| h.join().unwrap() as u64).sum()
}

fn solve_p2(elf_db: &str) -> u64 {
    // Ironically part 2 is far easier, just have to count the total amount of possible fresh
    // ingredients (sum each range)
    let (ranges, _) = elf_db.split_once("\n\n").unwrap();
    let ranges = create_merged_sorted_ranges(ranges);

    ranges.into_iter().map(|r| *r.end() - *r.start() + 1).sum()
}

aoc_main!(solve_p1, solve_p2);
