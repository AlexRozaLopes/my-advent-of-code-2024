use crate::day::day_one;

mod day;

fn main() {
    println!(
        "{}",
        format!(
            "DAY ONE: {}",
            day_one::distance_between(include_str!("data/day-one.txt"))
        )
    )
}
