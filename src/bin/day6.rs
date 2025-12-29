use advent_of_code_2025::aoc_main;

fn solve_p1(homework: &str) -> u64 {
    let homework_bytes = homework.trim().as_bytes();

    // Acumulators store results for each column & the op
    let mut accumulators: Vec<(u64, u8)> = Vec::with_capacity(20);

    // State-storing variables
    let mut multiplier = 1;
    let mut current_num = 0;
    let mut col_idx = 0;
    let mut parsing_ops = true;

    for &b in homework_bytes.iter().rev() {
        match b {
            b'\n' => {
                if parsing_ops {
                    parsing_ops = false;
                } else if multiplier > 1 {
                    apply_op(&mut accumulators, col_idx, current_num);
                    current_num = 0;
                    multiplier = 1;
                }

                col_idx = 0;
            }

            b'0'..=b'9' => {
                current_num += (b - b'0') as u64 * multiplier;
                multiplier *= 10;
            }

            b'+' | b'*' => {
                if parsing_ops {
                    let start_val = if b == b'+' { 0 } else { 1 };
                    accumulators.push((start_val, b));
                }
            }

            // For whitespace (any other option should leave the program unusable, this is to solve
            // a puzzle not for random dumb users)
            _ => {
                if !parsing_ops && multiplier > 1 {
                    apply_op(&mut accumulators, col_idx, current_num);
                    current_num = 0;
                    multiplier = 1;
                    col_idx += 1;
                }
            }
        }
    }

    // Handle first number (in input not iterator) not having newline before it
    if !parsing_ops && multiplier > 1 {
        apply_op(&mut accumulators, col_idx, current_num);
    }

    accumulators.iter().map(|(n, _)| *n).sum()
}

#[inline(always)]
fn apply_op(accs: &mut [(u64, u8)], idx: usize, num: u64) {
    if let Some((acc, op)) = accs.get_mut(idx) {
        match op {
            b'+' => *acc += num,
            b'*' => *acc *= num,
            _ => unreachable!(),
        }
    }
}

fn solve_p2(homework: &str) -> u64 {
    let rows: Vec<&[u8]> = homework.trim().lines().map(|l| l.as_bytes()).collect();

    let height = rows.len();
    let num_rows = height - 1;
    let op_row_idx = height - 1;

    let width = rows.iter().map(|r| r.len()).max().unwrap_or(0);

    // State for current block
    let mut block_numbers = Vec::new();
    let mut block_op = None;
    let mut grand_total = 0;

    for x in (0..width).rev() {
        let mut col_is_empty = true;
        let mut curr_col_val = 0;
        let mut has_digit = false;

        for y in 0..num_rows {
            // If line is short treat as space (for numbers with digits < height)
            let byte = rows[y].get(x).copied().unwrap_or(b' ');

            if byte.is_ascii_digit() {
                col_is_empty = false;
                has_digit = true;
                curr_col_val = curr_col_val * 10 + (byte - b'0') as u64;
            }
        }

        let op_byte = rows[op_row_idx].get(x).copied().unwrap_or(b' ');
        if matches!(op_byte, b'+' | b'*') {
            col_is_empty = false;
            block_op = Some(op_byte);
        }

        if col_is_empty {
            if !block_numbers.is_empty() {
                grand_total += resolve_block(&block_numbers, block_op);
                block_numbers.clear();
                block_op = None;
            }
        } else if has_digit {
            block_numbers.push(curr_col_val);
        }
    }

    if !block_numbers.is_empty() {
        grand_total += resolve_block(&block_numbers, block_op);
    }

    grand_total
}

fn resolve_block(numbers: &[u64], op: Option<u8>) -> u64 {
    match op.unwrap() {
        b'+' => numbers.iter().sum(),
        b'*' => numbers.iter().product(),
        _ => unreachable!(),
    }
}

aoc_main!(solve_p1, solve_p2);
