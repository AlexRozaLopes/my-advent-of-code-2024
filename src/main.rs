use crate::day::day_one;

mod day;

fn main() {
    println!(
        "Day 1: Historian Hysteria: PART ONE - ANSWER {}",
        day_one::distance_between(include_str!("data/day-one.txt"))
    );
    println!(
        "Day 1: Historian Hysteria: PART TWO - ANSWER {}",
        day_one::similarity_score(include_str!("data/day-one.txt"))
    )
}
