#[macro_export]
macro_rules! aoc_main {
    ($p1:ident, $p2:ident, p1_alt = $p1_alt:ident, p2_alt:ident) => {
        $crate::generate_main!($p1, $p2, $p1_alt, $p2_alt)
    };

    ($p1:ident, $p2:ident, p1_alt = $p1_alt:ident) => {
        $crate::generate_main!($p1, $p2, $p1_alt, $p2);
    };

    ($p1:ident, $p2:ident, p2_alt = $p2_alt:ident) => {
        $crate::generate_main!($p1, $p2, $p1, $p2_alt);
    };

    ($p1:ident, $p2:ident) => {
        $crate::generate_main!($p1, $p2, $p1, $p2);
    };
}

#[macro_export]
macro_rules! generate_main {
    ($p1:ident, $p2:ident, $p1_run:ident, $p2_run:ident) => {
        use std::fs;
        use std::time::Instant;

        fn main() -> Result<(), Box<dyn std::error::Error>> {
            let path = std::env::args().nth(1).expect("No path given");

            let use_alt = std::env::args().nth(2).map(|s| s == "1").unwrap_or(false);

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

            let mut total = String::new();

            for _ in 0..times {
                total = match (part, use_alt) {
                    (1, false) => $p1(&contents).to_string(),
                    (1, true) => $p1_run(&contents).to_string(),

                    (2, false) => $p2(&contents).to_string(),
                    (2, true) => $p2_run(&contents).to_string(),

                    _ => panic!("Invalid part number (must be 1 or 2)"),
                };
            }

            let total_end = start.elapsed();

            println!("Part: {}, Alternative/Optim: {}", part, use_alt);
            println!("Result: {}", total);
            println!("----------");
            println!("Total duration: {:?}", total_end);
            println!("IO duration:    {:?}", io_end);
            println!(
                "Compute (avg):  {:?}",
                ((total_end - io_end) / times as u32)
            );

            Ok(())
        }
    };
}
