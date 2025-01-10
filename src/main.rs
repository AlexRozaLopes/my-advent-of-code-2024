use crate::day::{day_five, day_four, day_one, day_three, day_two};

mod day;

fn main() {
    println!(
        "Day 1: Historian Hysteria: PART ONE - ANSWER {}",
        day_one::distance_between(include_str!("data/day-one.txt"))
    );
    println!(
        "Day 1: Historian Hysteria: PART TWO - ANSWER {}",
        day_one::similarity_score(include_str!("data/day-one.txt"))
    );
    println!(
        "Day 2: Red-Nosed Reports: PART ONE - ANSWER {}",
        day_two::reports_is_safe(include_str!("data/day-two.txt"))
    );
    println!(
        "Day 2: Red-Nosed Reports: PART TWO - ANSWER {}",
        day_two::reports_is_safe_with_one_erro(include_str!("data/day-two.txt"))
    );
    println!(
        "Day 3: Mull It Over: PART ONE - ANSWER {}",
        day_three::process(include_str!("data/day-three.txt"))
    );
    println!(
        "Day 3: Mull It Over: PART TWO - ANSWER {}",
        day_three::process_do_dont(include_str!("data/day-three.txt"))
    );
    println!("Day 4: Ceres Search: PART ONE - ANSWER {}",
    day_four::day_four_first_challenger());

    println!("Day 4: Ceres Search: PART TWO - ANSWER {}",
    day_four::day_four_second_challenger());

    println!("Day 5: Print Queue: PART ONE - ANSWER {}",
    day_five::print_queue(include_str!("data/day-five.txt")));

    println!("Day 6: Print Queue: PART TWO - ANSWER {}",
             day_five::show_fix_invalid_data(include_str!("data/day-five.txt")));
}
