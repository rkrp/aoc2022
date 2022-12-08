use std::{fs, collections::HashSet};

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_part1_solver() {
        let input1 :Vec<char> = "bvwbjplbgvbhsrlpgdmjqwftvncz".chars().collect();
        let input2 :Vec<char> = "nppdvjthqldpwncqszvftbrmjlhg".chars().collect();
        let input3 :Vec<char> = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".chars().collect();
        let input4 :Vec<char> = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".chars().collect();

        assert_eq!(solve_challenge(input1, 4), 5);
        assert_eq!(solve_challenge(input2, 4), 6);
        assert_eq!(solve_challenge(input3, 4), 10);
        assert_eq!(solve_challenge(input4, 4), 11);
    }

    #[test]
    fn test_part2_solver() {
        let input1 :Vec<char> = "mjqjpqmgbljsphdztnvjfqwrcgsmlb".chars().collect();
        let input2 :Vec<char> = "bvwbjplbgvbhsrlpgdmjqwftvncz".chars().collect();
        let input3 :Vec<char> = "nppdvjthqldpwncqszvftbrmjlhg".chars().collect();
        let input4 :Vec<char> = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".chars().collect();
        let input5 :Vec<char> = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".chars().collect();

        assert_eq!(solve_challenge(input1, 14), 19);
        assert_eq!(solve_challenge(input2, 14), 23);
        assert_eq!(solve_challenge(input3, 14), 23);
        assert_eq!(solve_challenge(input4, 14), 29);
        assert_eq!(solve_challenge(input5, 14), 26);
    }
}

fn parse_input(filepath: &str) -> Vec<char> {
    let data: String = fs::read_to_string(filepath).expect("Cannot read file");
    data.chars().collect()
}

fn solve_challenge(data: Vec<char>, winsize: usize) -> usize {
    let mut i = 0;

    while i < data.len() {
        let window = &data[i..i+winsize];
        let foo:HashSet<char> = HashSet::from_iter(window.iter().cloned());
        if window.len() == foo.len() {
            return i + winsize;
        }
        i+=1;
    }
    panic!("Invalid input");
}

pub fn part1() {
    let data = parse_input("./src/day06/input.txt");
    println!("DATA: {:?}", data);
    let result = solve_challenge(data, 4);
    
    println!("ANS: {}", result);
}

pub fn part2() {
    let data = parse_input("./src/day06/input.txt");
    let result = solve_challenge(data, 14);
    println!("ANS: {}", result);
}