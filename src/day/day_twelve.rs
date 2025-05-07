use std::collections::VecDeque;

pub fn part_one(input: &str) -> i32 {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let rows = grid.len();
    let cols = grid[0].len();
    let mut visited = vec![vec![false; cols]; rows];

    let mut total_price = 0;

    for i in 0..rows {
        for j in 0..cols {
            if !visited[i][j] {
                let plant = grid[i][j];
                let (area, perimeter) = bfs(&grid, &mut visited, i, j, plant);
                total_price += area * perimeter;
            }
        }
    }

    total_price
}

fn bfs(
    grid: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    start_i: usize,
    start_j: usize,
    plant: char,
) -> (i32, i32) {
    let mut area = 0;
    let mut perimeter = 0;

    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut queue = VecDeque::new();
    queue.push_back((start_i, start_j));
    visited[start_i][start_j] = true;

    while let Some((i, j)) = queue.pop_front() {
        area += 1;
        for (di, dj) in directions {
            let ni = i as isize + di;
            let nj = j as isize + dj;
            if ni < 0 || nj < 0 || ni >= grid.len() as isize || nj >= grid[0].len() as isize {
                perimeter += 1;
            } else {
                let ni = ni as usize;
                let nj = nj as usize;
                if grid[ni][nj] != plant {
                    perimeter += 1;
                } else if !visited[ni][nj] {
                    visited[ni][nj] = true;
                    queue.push_back((ni, nj));
                }
            }
        }
    }

    (area, perimeter)
}


#[test]
fn part_1_test() {
    let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
    
    assert_eq!(part_one(input), 1930);
}
