use std::collections::{VecDeque, HashSet};

pub fn parse_map(input: &str) -> Vec<Vec<u8>> {
    input.lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
        .collect()
}

pub fn find_trailheads(map: &[Vec<u8>]) -> Vec<(usize, usize)> {
    let mut trailheads = Vec::new();
    for (row, line) in map.iter().enumerate() {
        for (col, &height) in line.iter().enumerate() {
            if height == 0 {
                trailheads.push((row, col));
            }
        }
    }
    trailheads
}

pub fn count_reachable_nines(map: &[Vec<u8>], start: (usize, usize)) -> usize {
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut reachable_nines = HashSet::new();

    queue.push_back(start);
    visited.insert(start);

    while let Some((r, c)) = queue.pop_front() {
        let current_height = map[r][c];

        for &(dr, dc) in &directions {
            let new_r = r as isize + dr;
            let new_c = c as isize + dc;

            if new_r >= 0 && new_c >= 0 {
                let new_r = new_r as usize;
                let new_c = new_c as usize;

                if new_r < map.len() && new_c < map[0].len() {
                    let new_height = map[new_r][new_c];

                    if new_height == current_height + 1 && visited.insert((new_r, new_c)) {
                        queue.push_back((new_r, new_c));
                        if new_height == 9 {
                            reachable_nines.insert((new_r, new_c));
                        }
                    }
                }
            }
        }
    }

    reachable_nines.len()
}

pub fn total_trailhead_score(input: &str) -> usize {
    let map = parse_map(input);
    let trailheads = find_trailheads(&map);

    trailheads.iter()
        .map(|&start| count_reachable_nines(&map, start))
        .sum()
}

#[test]
fn test() {
    let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"; // Substitua pelo seu input real

    let result = total_trailhead_score(input);
    println!("Soma dos scores dos trailheads: {}", result);
    assert_eq!(0,0)
}
