pub fn day_four_first_challenger() -> usize {
    let input: Vec<Vec<char>> = include_str!("../data/day-four.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let target = ['M', 'A', 'S'];

    let height = input.len();
    let width = input.first().unwrap().len();
    let mut ans = 0;
    for i in 0..height {
        for j in 0..width {
            if input[i][j] != 'X' {
                continue;
            }

            if j > 2 && input[i][j - 3..j] == ['S', 'A', 'M'] {
                ans += 1;
            }
            if j < width - 3 && input[i][j + 1..j + 4] == target {
                ans += 1;
            }
            if i > 2 && (1..4).all(|k| input[i - k][j] == target[k - 1]) {
                ans += 1;
            }
            if i < height - 3 && (1..4).all(|k| input[i + k][j] == target[k - 1]) {
                ans += 1;
            }
            if i > 2 && j > 2 && (1..4).all(|k| input[i - k][j - k] == target[k - 1]) {
                ans += 1;
            }
            if i < height - 3 && j > 2 && (1..4).all(|k| input[i + k][j - k] == target[k - 1]) {
                ans += 1;
            }
            if i > 2 && j < width - 3 && (1..4).all(|k| input[i - k][j + k] == target[k - 1]) {
                ans += 1;
            }
            if i < height - 3
                && j < width - 3
                && (1..4).all(|k| input[i + k][j + k] == target[k - 1])
            {
                ans += 1;
            }
        }
    }
    ans
}

pub fn day_four_second_challenger() -> usize {
    let input: Vec<Vec<char>> = include_str!("../data/day-four.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let height = input.len();
    let width = input.first().unwrap().len();
    let mut ans = 0;
    for i in 1..height - 1 {
        for j in 1..width - 1 {
            if input[i][j] != 'A' {
                continue;
            }

            if (input[i - 1][j - 1] == 'M' && input[i + 1][j + 1] == 'S'
                || input[i - 1][j - 1] == 'S' && input[i + 1][j + 1] == 'M')
                && (input[i - 1][j + 1] == 'M' && input[i + 1][j - 1] == 'S'
                || input[i - 1][j + 1] == 'S' && input[i + 1][j - 1] == 'M')
            {
                ans += 1;
            }
        }
    }
    ans
}