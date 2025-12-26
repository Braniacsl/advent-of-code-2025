use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> std::io::Result<()> {
    let f = File::open("data/day1-password")?;
    let reader = BufReader::new(f);

    let mut dial_num: usize = 50;
    let mut orig_count: usize = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let direction = line.chars().next().unwrap();
        let amount: usize = line[1..].parse().unwrap();

        orig_count += amount / 100;
        let amount = amount % 100;

        match direction {
            'L' => {
                orig_count += if dial_num != 0 && amount >= dial_num {
                    1
                } else {
                    0
                };
                dial_num = (dial_num + 100 - amount) % 100;
            }
            'R' => {
                orig_count += if (amount + dial_num) >= 100 { 1 } else { 0 };
                dial_num = (dial_num + amount) % 100;
            }
            _ => panic!("Unknown direction"),
        }
    }

    println!("{}", orig_count);

    Ok(())
}
