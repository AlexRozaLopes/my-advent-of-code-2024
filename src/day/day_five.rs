use std::collections::HashMap;

pub fn print_queue(input: &str) -> usize {
    let data_valid = show_data_valid(input);

    data_valid.iter().map(|n|n.parse::<usize>().unwrap_or(0)).fold(0, |acc, n:usize| acc + n)
}

fn split_in_list_rules_and_data(input: &str) -> (Vec<&str>,Vec<&str>) {
    let (rules,data):(Vec<&str>,Vec<&str>) = input.lines().partition(|l| l.contains('|'));

    println!("rules: {:?}", rules);
    println!("data: {:?}", data);
    (rules,data)
}

fn format_rules(rules: Vec<&str>) -> HashMap<&str, Vec<&str>> {
    let mut map: HashMap<&str,Vec<&str>> = HashMap::new();
    rules.iter().for_each(|l| {
        let split = l.split("|").collect::<Vec<&str>>();
        map.entry(split[0]).and_modify(|v|v.push(split[1])).or_insert(vec![split[1]]);

    });
    println!("rules formated: {:?}", map);
    map
}

fn show_data_valid(input: &str) -> Vec<&str> {
    let (rules, data) = split_in_list_rules_and_data(input);
    let formated_rules = format_rules(rules);

    let mut data_valid: Vec<&str> = Vec::new();

    data.iter().for_each(|l| {
        let mut is_valid = true;
        let numbers = l.split(",").collect::<Vec<&str>>();
        for i in 0..numbers.len() {
            if let Some(v) = formated_rules.get(&numbers[i]) {
                if i + 1 < numbers.len() {
                    is_valid = is_valid && v.contains(&numbers[i + 1]);
                } else if i > 0 {
                    is_valid = is_valid && !v.contains(&numbers[i - 1]);
                }
            }


        }
        if is_valid {
            println!("line numbers: {:?}", numbers);
            data_valid.push(numbers[numbers.len() /2]);
        }
    });
    data_valid
}

#[test]
fn days_five_examples() {
    let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    assert_eq!(print_queue(input),143)
}