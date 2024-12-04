use regex::Regex;

pub fn process(input: &str) -> u32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let result = re
        .captures_iter(input)
        .map(|caps| {
            let first = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let second = caps.get(2).unwrap().as_str().parse::<u32>().unwrap();
            first * second
        })
        .sum::<u32>();

    result
}

pub fn process_do_dont(input: &str) -> u32 {
    let mut segments = input.split("don't()");
    let initial = process(segments.next().unwrap_or(""));
    let part_two = segments.fold(initial, |acc, segment| {
        acc + process(&segment[segment.find("do()").unwrap_or(segment.len())..])
    });
    part_two
}

#[test]
fn test_process() {
    let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    assert_eq!(process(input), 161);
}

#[test]
fn test_process_with_do() {
    let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    assert_eq!(process_do_dont(input), 48);
}
