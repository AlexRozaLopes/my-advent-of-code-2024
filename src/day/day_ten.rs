use std::collections::{HashSet, HashMap, VecDeque};

type Point = (usize, usize);

pub fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| line.chars().map(|c| c as u8 - b'0').collect()).collect()
}

fn neighbors((x, y): Point, height: usize, width: usize) -> Vec<Point> {
    let mut result = vec![];
    if x > 0 { result.push((x - 1, y)); }
    if y > 0 { result.push((x, y - 1)); }
    if x + 1 < height { result.push((x + 1, y)); }
    if y + 1 < width { result.push((x, y + 1)); }
    result
}

// Parte 1
fn score_trailhead(map: &[Vec<u8>], start: Point) -> usize {
    let (h, w) = (map.len(), map[0].len());
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(start);
    let mut found = HashSet::new();

    while let Some((x, y)) = queue.pop_front() {
        if visited.contains(&(x, y)) {
            continue;
        }
        visited.insert((x, y));
        let current = map[x][y];

        if current == 9 {
            found.insert((x, y));
            continue;
        }

        for (nx, ny) in neighbors((x, y), h, w) {
            if map[nx][ny] == current + 1 {
                queue.push_back((nx, ny));
            }
        }
    }

    found.len()
}

// Parte 2
fn count_paths(map: &[Vec<u8>], pos: Point, cache: &mut HashMap<Point, usize>) -> usize {
    if map[pos.0][pos.1] == 9 {
        return 1;
    }
    if let Some(&cached) = cache.get(&pos) {
        return cached;
    }

    let (h, w) = (map.len(), map[0].len());
    let mut count = 0;
    let current = map[pos.0][pos.1];
    for (nx, ny) in neighbors(pos, h, w) {
        if map[nx][ny] == current + 1 {
            count += count_paths(map, (nx, ny), cache);
        }
    }
    cache.insert(pos, count);
    count
}

pub fn part1(map: &[Vec<u8>]) -> usize {
    let mut total = 0;
    for x in 0..map.len() {
        for y in 0..map[0].len() {
            if map[x][y] == 0 {
                total += score_trailhead(map, (x, y));
            }
        }
    }
    total
}

pub fn part2(map: &[Vec<u8>]) -> usize {
    let mut total = 0;
    for x in 0..map.len() {
        for y in 0..map[0].len() {
            if map[x][y] == 0 {
                let mut cache = HashMap::new();
                total += count_paths(map, (x, y), &mut cache);
            }
        }
    }
    total
}


#[test]
fn test_example_map() {
    let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    let map = parse_input(input);
    assert_eq!(part1(&map), 36);
    assert_eq!(part2(&map), 81);
}
