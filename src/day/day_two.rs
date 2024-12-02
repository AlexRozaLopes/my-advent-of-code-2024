use std::ops::Sub;

#[test]
fn test_count_reports_is_safe(){
    let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
    assert_eq!(2,reports_is_safe(input));
}

pub fn reports_is_safe(input: &str) -> i32 {
    let mut count_valid = 0;
    input.lines().for_each(|line| {
        let numbers = line.split_whitespace().map(|number| { number.parse::<i32>().unwrap() }).collect::<Vec<i32>>();
        let mut count = 2;
        for i in 0..numbers.len() {
            if i !=0 {
                if i != (numbers.len()-1) {
                    if numbers[i].sub(&numbers[i-1]) <= 3 && numbers[i].sub(&numbers[i-1]).is_positive() && numbers[i+1].sub(&numbers[i]) <= 3 && numbers[i+1].sub(&numbers[i]).is_positive() {
                        count += 1;
                    }
                    if numbers[i].sub(&numbers[i-1]) >= -3 && numbers[i].sub(&numbers[i-1]).is_negative() && numbers[i+1].sub(&numbers[i]) >= -3 && numbers[i+1].sub(&numbers[i]).is_negative() {
                        count += 1;
                    }
                }
            }

        }
        if count == numbers.len() {
            println!("report: {:?} is valid" ,numbers);
            count_valid +=1;
        }
    });
    count_valid
}