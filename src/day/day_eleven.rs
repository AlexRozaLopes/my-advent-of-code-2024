use std::collections::HashMap;

pub fn solve(input: &str, steps: usize) -> usize {
    let mut stones: HashMap<u64, usize> = input
        .trim()
        .split_ascii_whitespace()
        .map(|s| (s.parse().unwrap(), 1))
        .collect();
    for _ in 0..steps {
        for (stone, n) in stones.drain().collect::<Vec<_>>() {
            let mut insert = |s| {
                stones.entry(s).and_modify(|x| *x += n).or_insert(n);
            };
            if stone == 0 {
                insert(1);
            } else {
                match (stone as f32).log10().floor() as u32 + 1 {
                    digits if digits % 2 == 0 => {
                        insert(stone / 10u64.pow(digits / 2));
                        insert(stone % 10u64.pow(digits / 2));
                    }
                    _ => insert(stone * 2024),
                }
            }
        }
    }
    stones.values().sum()
}
