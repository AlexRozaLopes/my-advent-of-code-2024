use crate::day::{day_one, day_two};

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
    )
}
