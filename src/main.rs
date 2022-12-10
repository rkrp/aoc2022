
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;

fn main() {
    let mode = String::from("d9p1");
    match &mode[..] {
        "d1p1" => day01::day01::part1(),
        "d1p2" => day01::day01::part2(),
        "d2p1" => day02::day02::part1(),
        "d2p2" => day02::day02::part2(),
        "d3p1" => day03::day03::part1(),
        "d3p2" => day03::day03::part2(),
        "d4p1" => day04::day04::part1(),
        "d4p2" => day04::day04::part2(),
        "d5p1" => day05::day05::part1(),
        "d5p2" => day05::day05::part2(),
        "d6p1" => day06::day06::part1(),
        "d6p2" => day06::day06::part2(),
        "d7p1" => day07::day07::part1(),
        "d7p2" => day07::day07::part2(),
        "d8p1" => day08::day08::part1(),
        "d8p2" => day08::day08::part2(),
        "d9p1" => day09::day09::part1(),
        "d9p2" => day09::day09::part2(),
        _ => println!("Invalid mode!"),
    }
}