
mod day01;
mod day02;
mod day03;
mod day04;

fn main() {
    let mode = String::from("d4p2");
    match &mode[..] {
        "d1p1" => day01::day01::part1(),
        "d1p2" => day01::day01::part2(),
        "d2p1" => day02::day02::part1(),
        "d2p2" => day02::day02::part2(),
        "d3p1" => day03::day03::part1(),
        "d3p2" => day03::day03::part2(),
        "d4p1" => day04::day04::part1(),
        "d4p2" => day04::day04::part2(),
        _ => println!("Invalid mode!"),
    }
}
