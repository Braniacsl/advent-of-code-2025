use std::collections::{HashMap, HashSet, VecDeque};

use advent_of_code_2025::aoc_main;

type Point = (usize, usize);

fn parse(diagram: &str) -> (HashMap<usize, Vec<usize>>, Option<Point>, usize) {
    // We extract coords of splitters and store in hashmap with columns as the key,
    // and rows sorted for easy lookup
    let mut splitters: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut start_pos: Option<Point> = None;
    let mut width = 0;

    for (y, line) in diagram.lines().enumerate() {
        width = width.max(line.len());
        for (x, char) in line.chars().enumerate() {
            match char {
                '^' => {
                    splitters.entry(x).or_default().push(y);
                }
                'S' => start_pos = Some((x, y)),
                _ => {}
            }
        }
    }

    for rows in splitters.values_mut() {
        // Sort unstable because initial order doesnt matter, as long as its sorted
        rows.sort_unstable();
    }

    (splitters, start_pos, width)
}

fn solve_p1(diagram: &str) -> usize {
    let (splitters, start_pos, width) = parse(diagram);

    let start = start_pos.expect("No start pos found");

    // For DFS traversal we only need to count splitters hit
    let mut activated_splitters: HashSet<Point> = HashSet::new();

    // Queue stores beams to calc
    let mut queue = VecDeque::new();
    queue.push_back(start);

    while let Some((x, current_y)) = queue.pop_front() {
        if let Some(col_splitters) = splitters.get(&x) {
            // Get splitter directly below
            let idx = col_splitters.partition_point(|&row| row <= current_y);

            if idx < col_splitters.len() {
                let hit_y = col_splitters[idx];
                let splitter_pos = (x, hit_y);

                if activated_splitters.insert(splitter_pos) {
                    // Add new beams to queue
                    // left boundary check
                    if x > 0 {
                        queue.push_back((x - 1, hit_y));
                    }
                    // right boundary check
                    if x + 1 < width {
                        queue.push_back((x + 1, hit_y));
                    }
                }
            }
        }
    }

    activated_splitters.len()
}

fn solve_p2(diagram: &str) -> u64 {
    let (splitters, start_pos, width) = parse(diagram);
    let start = start_pos.expect("No start pos");

    // Memoization cache so recursive calculation doesnt repeat subtrees. Key is Point of hit
    // splitter and val is # of timelines that spawn from this splitter onward
    let mut memo: HashMap<Point, u64> = HashMap::new();

    fn count_timelines(
        x: usize,
        y: usize,
        width: usize,
        splitters: &HashMap<usize, Vec<usize>>,
        memo: &mut HashMap<Point, u64>,
    ) -> u64 {
        if let Some(col) = splitters.get(&x) {
            let idx = col.partition_point(|&row| row <= y);

            if idx < col.len() {
                let hit_y = col[idx];
                let splitter_pos = (x, hit_y);

                // Check if subtree searched already
                if let Some(&count) = memo.get(&splitter_pos) {
                    return count;
                }

                let mut total_branches = 0;

                // left
                if x > 0 {
                    total_branches += count_timelines(x - 1, hit_y, width, splitters, memo);
                } else {
                    total_branches += 1;
                }

                // right
                if x + 1 < width {
                    total_branches += count_timelines(x + 1, hit_y, width, splitters, memo);
                } else {
                    total_branches += 1;
                }

                memo.insert(splitter_pos, total_branches);
                return total_branches;
            }
        }

        // Base case
        1
    }

    count_timelines(start.0, start.1, width, &splitters, &mut memo)
}

aoc_main!(solve_p1, solve_p2);
