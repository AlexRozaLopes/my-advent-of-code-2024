use crate::day::{day_eight, day_five, day_four, day_nine, day_one, day_seven, day_six, day_ten, day_three, day_two};

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
    println!(
        "Day 4: Ceres Search: PART ONE - ANSWER {}",
        day_four::day_four_first_challenger()
    );

    println!(
        "Day 4: Ceres Search: PART TWO - ANSWER {}",
        day_four::day_four_second_challenger()
    );

    println!(
        "Day 5: Print Queue: PART ONE - ANSWER {}",
        day_five::print_queue(include_str!("data/day-five.txt"))
    );

    println!(
        "Day 5: Print Queue: PART TWO - ANSWER {}",
        day_five::show_fix_invalid_data(include_str!("data/day-five.txt"))
    );

    println!(
        "Day 6: Guard Gallivant: PART ONE - ANSWER {}",
        day_six::part1(include_str!("data/day-six.txt"))
    );

    println!(
        "Day 6: Guard Gallivant: PART TWO - ANSWER {}",
        day_six::part2(include_str!("data/day-six.txt"))
    );

    println!(
        "Day 7: Bridge Repair: PART ONE - ANSWER {}",
        day_seven::bridge_repair(include_str!("data/day-seven.txt"), false)
    );

    println!(
        "Day 7: Bridge Repair: PART TWO - ANSWER {}",
        day_seven::bridge_repair(include_str!("data/day-seven.txt"), true)
    );

    day_eight::main();

    println!(
        "Day 9: Disk Fragmenter: PART ONE - ANSWER {}",
        day_nine::part1(include_str!("data/day-nine.txt"))
    );

    println!(
        "Day 9: Disk Fragmenter: PART TWO - ANSWER {}",
        day_nine::part2(include_str!("data/day-nine.txt"))
    );

    println!("Day 10: Hoof It: PART ONE - ANSWER {}", day_ten::total_trailhead_score(include_str!("data/day-ten.txt")))
}
