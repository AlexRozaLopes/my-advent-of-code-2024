use std::ops::Sub;
use std::str::Lines;

#[test]
fn test_distance_between() {
    let input = "3   4
4   3
2   5
1   3
3   9
3   3";

    assert_eq!(11, distance_between(input))
}

pub fn distance_between(input: &str) -> i32 {
    let (list_left, list_right) = split_in_list(input.lines());
    let mut list_sum = Vec::<i32>::new();

    for i in 0..list_left.len() {
        let number = list_left[i].sub(list_right[i]).abs();
        println!("{}", number);
        list_sum.push(number);
    }
    list_sum.iter().sum()
}

fn split_in_list(lines: Lines) -> (Vec<i32>, Vec<i32>) {
    let mut list_left: Vec<i32> = Vec::new();
    let mut list_right: Vec<i32> = Vec::new();
    lines.for_each(|line| {
        let split_values = line.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        list_left.push(split_values[0]);
        list_right.push(split_values[1]);
    });
    list_left.sort();
    list_right.sort();
    println!("{:?}", list_left);
    println!("{:?}", list_right);
    (list_left, list_right)
}
