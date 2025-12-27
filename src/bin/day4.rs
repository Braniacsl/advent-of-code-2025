use std::{fs, io, time::Instant};

fn is_at(char: Option<&u8>) -> usize {
    match char {
        Some(&b'@') => 1,
        _ => 0,
    }
}

// naive-solution. loop through char matrix and check 8 surrounding positions
// using index math
fn solve_p1(diagram: &str) -> Option<usize> {
    let diagram: Vec<Vec<u8>> = diagram.lines().map(|s| s.as_bytes().to_vec()).collect();

    let mut total = 0;
    for i in 0..diagram.len() {
        let line = diagram.get(i)?;
        for j in 0..line.len() {
            let char = line.get(j)?;

            // Skip if not '@'
            if is_at(Some(char)) == 0 {
                continue;
            }

            let mut neighbor_count = 0;
            // Top Row
            neighbor_count += is_at(
                diagram
                    .get(i.wrapping_sub(1))
                    .and_then(|l| l.get(j.wrapping_sub(1))),
            ); // Top-Left
            neighbor_count += is_at(diagram.get(i.wrapping_sub(1)).and_then(|l| l.get(j))); // Top-Center
            neighbor_count += is_at(diagram.get(i.wrapping_sub(1)).and_then(|l| l.get(j + 1))); // Top-Right

            // Middle Row
            neighbor_count += is_at(diagram.get(i).and_then(|l| l.get(j.wrapping_sub(1)))); // Left
            neighbor_count += is_at(diagram.get(i).and_then(|l| l.get(j + 1))); // Right

            // Bottom Row
            neighbor_count += is_at(diagram.get(i + 1).and_then(|l| l.get(j.wrapping_sub(1)))); // Bottom-Left
            neighbor_count += is_at(diagram.get(i + 1).and_then(|l| l.get(j))); // Bottom-Center
            neighbor_count += is_at(diagram.get(i + 1).and_then(|l| l.get(j + 1))); // Bottom-Right

            if neighbor_count < 4 {
                total += 1
            }
        }
    }

    Some(total)
}

fn solve_p1_flattened(diagram: &str) -> usize {
    let width = diagram.lines().next().unwrap().len();
    let height = diagram.lines().count();

    // Create flattened diagram. Added padding to avoid boundary checks (thus avoiding branches)
    let stride = width + 2;
    let mut grid = vec![b'.'; stride * (height + 2)];

    // Copy values from diagram
    for (row_idx, line) in diagram.lines().enumerate() {
        // Skip borders with +1
        let start = (row_idx + 1) * stride + 1;
        grid[start..start + width].copy_from_slice(line.as_bytes());
    }

    let s = stride as isize;
    let offset = [-s - 1, -s, -s + 1, -1, 1, s - 1, s, s + 1];

    // Idiomatic rust solution using iterators
    // Ends up being not so fast. To take full advantaged of flattened array (which provides cache
    // locality) we need manually run loops and unroll some of them too(plus we unrolled in naive
    // already).
    // (1..=height)
    //     .flat_map(|y| {
    //         let row_start = y * stride + 1;
    //         let row_end = row_start + width;
    //         row_start..row_end
    //     })
    //     .filter(|&i| grid[i] == b'@')
    //     .filter(|&i| {
    //         let neighbor_count = offset
    //             .iter()
    //             // We can add offset without worrying about overflow because of padding
    //             .filter(|&&offset| grid[(i as isize + offset) as usize] == b'@')
    //             .count();
    //
    //         neighbor_count < 4
    //     })
    //     .count()

    // Avoid rust abstractions, handle flattened array calculation iteration
    // & checks manually
    let mut count = 0;

    for y in 1..=height {
        let row_start = y * stride + 1;
        for i in row_start..(row_start + width) {
            if grid[i] != b'@' {
                continue;
            }

            // manually unroll loop (done by filter before) like the naive approach
            let neighbors = (grid[(i as isize + offset[0]) as usize] == b'@') as usize
                + (grid[(i as isize + offset[1]) as usize] == b'@') as usize
                + (grid[(i as isize + offset[2]) as usize] == b'@') as usize
                + (grid[(i as isize + offset[3]) as usize] == b'@') as usize
                + (grid[(i as isize + offset[4]) as usize] == b'@') as usize
                + (grid[(i as isize + offset[5]) as usize] == b'@') as usize
                + (grid[(i as isize + offset[6]) as usize] == b'@') as usize
                + (grid[(i as isize + offset[7]) as usize] == b'@') as usize;

            if neighbors < 4 {
                count += 1;
            }
        }
    }
    count
}

fn solve_p2(diagram: &str) -> usize {
    let width = diagram.lines().next().unwrap().len();
    let height = diagram.lines().count();

    // Create flattened diagram. Added padding to avoid boundary checks (thus avoiding branches)
    let stride = width + 2;
    let mut grid = vec![b'.'; stride * (height + 2)];

    // Copy values from diagram
    for (row_idx, line) in diagram.lines().enumerate() {
        // Skip borders with +1
        let start = (row_idx + 1) * stride + 1;
        grid[start..start + width].copy_from_slice(line.as_bytes());
    }

    let s = stride as isize;
    let offsets = [-s - 1, -s, -s + 1, -1, 1, s - 1, s, s + 1];

    // Setup queue to check. We can treat this as a flood fill problem because paper rolls that are
    // neighbors of the initial count are the only ones that need rechecking.
    let mut queue: Vec<usize> = Vec::with_capacity(width * height / 4);
    let mut total_removed = 0;

    // Perform first check to fill queue
    for y in 1..=height {
        let row_start = y * stride + 1;
        for i in row_start..(row_start + width) {
            if grid[i] != b'@' {
                continue;
            }

            // manually unroll loop (done by filter before) like the naive approach
            let neighbors = (grid[(i as isize + offsets[0]) as usize] == b'@') as usize
                + (grid[(i as isize + offsets[1]) as usize] == b'@') as usize
                + (grid[(i as isize + offsets[2]) as usize] == b'@') as usize
                + (grid[(i as isize + offsets[3]) as usize] == b'@') as usize
                + (grid[(i as isize + offsets[4]) as usize] == b'@') as usize
                + (grid[(i as isize + offsets[5]) as usize] == b'@') as usize
                + (grid[(i as isize + offsets[6]) as usize] == b'@') as usize
                + (grid[(i as isize + offsets[7]) as usize] == b'@') as usize;

            if neighbors < 4 {
                grid[i] = b'.';
                queue.push(i);
                total_removed += 1;
            }
        }
    }

    // Now we only need to perform operation on items in queue until its empty
    // rather than rechecking the entire diagram each time
    while let Some(idx) = queue.pop() {
        for &offset in &offsets {
            let neighbor_idx = (idx as isize + offset) as usize;

            if grid[neighbor_idx] == b'@' {
                let neighbors = (grid[(neighbor_idx as isize + offsets[0]) as usize] == b'@')
                    as usize
                    + (grid[(neighbor_idx as isize + offsets[1]) as usize] == b'@') as usize
                    + (grid[(neighbor_idx as isize + offsets[2]) as usize] == b'@') as usize
                    + (grid[(neighbor_idx as isize + offsets[3]) as usize] == b'@') as usize
                    + (grid[(neighbor_idx as isize + offsets[4]) as usize] == b'@') as usize
                    + (grid[(neighbor_idx as isize + offsets[5]) as usize] == b'@') as usize
                    + (grid[(neighbor_idx as isize + offsets[6]) as usize] == b'@') as usize
                    + (grid[(neighbor_idx as isize + offsets[7]) as usize] == b'@') as usize;

                if neighbors < 4 {
                    grid[neighbor_idx] = b'.';
                    queue.push(neighbor_idx);
                    total_removed += 1;
                }
            }
        }
    }

    total_removed
}

fn main() -> io::Result<()> {
    let path = std::env::args().nth(1).expect("No path given");
    let optim = std::env::args().nth(2).map(|s| s == "1").unwrap_or(false);
    let times: usize = std::env::args()
        .nth(3)
        .unwrap_or("1".to_string())
        .parse()
        .expect("Times NaN");
    let part: usize = std::env::args()
        .nth(4)
        .expect("no part number")
        .parse()
        .expect("invalid part");

    let start = Instant::now();

    let contents = fs::read_to_string(path)?;

    let io_end = start.elapsed();

    let mut total = 0;
    for _ in 0..times {
        total = match (part, optim) {
            (1, false) => solve_p1(&contents).expect("No total"),
            (1, true) => solve_p1_flattened(&contents),
            (2, _) => solve_p2(&contents),
            _ => panic!("Invalid param values"),
        };
    }

    let total_end = start.elapsed();

    println!("Chose: {}", optim);
    println!("Total Paper rolls: {}", total);
    println!("Total Duration: {:?}", total_end);
    println!("IO Duration: {:?}", io_end);
    println!(
        "Actual Compute Duration: {:?}",
        ((total_end - io_end) / times as u32)
    );

    Ok(())
}
