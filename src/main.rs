
mod day01;
mod day02;

fn main() {
    let mode = String::from("d2p2");
    match &mode[..] {
        "d1p1" => day01::day01::part1(),
        "d1p2" => day01::day01::part2(),
        "d2p1" => day02::day02::part1(),
        "d2p2" => day02::day02::part2(),
        _ => println!("Invalid mode!"),
    }
}
