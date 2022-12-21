use std::{fs, cmp::Ordering};
use serde_json::{Value, json};

#[derive(PartialEq,Eq)]
#[derive(Debug)]
enum CompareResult {
    SMALL,
    LARGE,
    EQUAL,
}

fn parse_input(filepath: &str) -> Vec<(String, String)> {
    let data: String = fs::read_to_string(filepath).expect("Cannot read file");
    let data:Vec<(String, String)> = data
                    .trim()
                    .split("\n\n")
                    .map(|x| (x.split("\n").next().unwrap().to_owned(), x.split("\n").last().unwrap().to_owned()))
                    .collect();
    
    data
}

fn parse_packets(filepath: &str) -> Vec<String> {
    let data: String = fs::read_to_string(filepath).expect("Cannot read file");
    let data:Vec<String> = data
                    .trim()
                    .replace("\n\n", "\n")
                    .split("\n")
                    .map(|x| x.to_string())
                    .collect();        
    data
}

fn compare(op1: &Value, op2: &Value, level:usize) -> CompareResult {
    if op1.is_i64() && op2.is_i64() {
        let op1 = op1.as_i64().unwrap();
        let op2 = op2.as_i64().unwrap();
        //println!("{}- Compare {} vs {}", " ".repeat(level * 2), op1, op2);

        if op1 == op2 {
            return CompareResult::EQUAL;
        }
        if op1 < op2 {
            return CompareResult::SMALL;
        }
        return CompareResult::LARGE;
    }

    if op1.is_array() && op2.is_array() {
        let op1 = op1.as_array().unwrap();
        let op2 = op2.as_array().unwrap();
        //println!("{}- Compare {} vs {}", " ".repeat(level * 2), serde_json::to_string(op1).unwrap(), serde_json::to_string(op2).unwrap());

        let minlen = if op1.len() <= op2.len() {op1.len()} else {op2.len()};

        for i in 0..minlen {
            let ret = compare(&op1[i], &op2[i], level + 1);
            if ret == CompareResult::EQUAL {
                continue;
            }
            return ret;
        }

        if op1.len() < op2.len() {
            return CompareResult::SMALL;
        } else if op1.len() > op2.len() {
            return CompareResult::LARGE;
        }

        return CompareResult::EQUAL;
    } else if op1.is_array() && op2.is_i64() {
        if op1.as_array().unwrap().len() == 0 {
            //println!("{}- Left side ran out of items, so inputs are in the right order", " ".repeat(level * 2));
            return CompareResult::SMALL;
        }

        let op2 = json!(vec![op2]);
        println!("{}- Compare {} vs {}", " ".repeat(level * 2), serde_json::to_string(op1).unwrap(), op2);
        return compare(op1, &op2, level + 1);
    } else if op2.is_array() && op1.is_i64() {
        if op2.as_array().unwrap().len() == 0 {
            //println!("{}- Right side ran out of items, so inputs are not in the right order", " ".repeat(level * 2));
            return CompareResult::LARGE;
        }

        let op1 = json!(vec![op1]);
        println!("{}- Compare {} vs {}", " ".repeat(level * 2), op1, serde_json::to_string(op2).unwrap());
        return compare(&op1, op2, level +1);
    } else {
        panic!("- Unexpected objects received: {:?} and {:?}", op1, op2);
    }

}

fn solve_one(op1:&str, op2:&str) -> CompareResult {
    let op1:Value = serde_json::from_str(op1).unwrap();
    let op2:Value = serde_json::from_str(op2).unwrap();
    let level = 0;
    
    return compare(&op1, &op2, level + 1);
}

fn packet_cmp(op1:&str, op2:&str) -> Ordering {
    match solve_one(op1, op2) {
        CompareResult::EQUAL => Ordering::Equal,
        CompareResult::LARGE => Ordering::Greater,
        CompareResult::SMALL => Ordering::Less,
    }
}

pub fn part1() {
    let problems = parse_input("./src/day13/input.txt");
    let mut result = 0;
    for i in 0..problems.len() {
        let (op1, op2) = &problems[i];
        if solve_one(op1, op2) == CompareResult::SMALL {
            result += i + 1;
        }
    }
    println!("Result: {}", result);
}

pub fn part2() {
    let mut packets = parse_packets("./src/day13/input.txt");
    // Add divider packets 
    packets.push("[[2]]".to_string());
    packets.push("[[6]]".to_string());

    packets.sort_by(|a, b| packet_cmp(a, b));

    // Find the divider packets
    let start = packets.iter().position(|x| **x == "[[2]]".to_string()).unwrap();
    let end = packets.iter().position(|x| **x == "[[6]]".to_string()).unwrap();

    println!("Result :{}", (start + 1) * (end + 1));
    

    // let result = do_monkey_business(monkeys, 20);
    // println!("ANS: {}", result);
}

#[cfg(test)]
mod tests {

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    //use super::*;

    use super::*;

    #[test]
    fn test_part1() {
        let result = solve_one("[1,1,3,1,1]", "[1,1,5,1,1]");
        assert_eq!(result, CompareResult::SMALL);

        let result = solve_one("[[1],[2,3,4]]", "[[1],4]");
        assert_eq!(result, CompareResult::SMALL);

        let result = solve_one("[9]", "[[8,7,6]]");
        assert_eq!(result, CompareResult::LARGE);

        let result = solve_one("[[4,4],4,4]", "[[4,4],4,4,4]");
        assert_eq!(result, CompareResult::SMALL);

        let result = solve_one("[7,7,7,7]", "[7,7,7]");
        assert_eq!(result, CompareResult::LARGE);

        let result = solve_one("[]", "[3]");
        assert_eq!(result, CompareResult::SMALL);

        let result = solve_one("[[[]]]", "[[]]");
        assert_eq!(result, CompareResult::LARGE);

        let result = solve_one("[1,[2,[3,[4,[5,6,7]]]],8,9]", "[1,[2,[3,[4,[5,6,0]]]],8,9]");
        assert_eq!(result, CompareResult::LARGE);
    }

    #[test]
    fn test_part2() {

    }

    #[test]
    fn operation_test() {

    }
}