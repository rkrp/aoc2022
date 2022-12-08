use std::fs;
use regex::Regex;

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_part1_parser() {
        let (stacks, ops) = parse_input("./src/day05/sample_input.txt");
        assert_eq!(stacks[0], vec!['Z', 'N']);
        assert_eq!(stacks[1], vec!['M', 'C', 'D']);
        assert_eq!(stacks[2], vec!['P']);

        assert_eq!(ops[0], StackOp{num: 1, from: 2, to: 1});
        assert_eq!(ops[1], StackOp{num: 3, from: 1, to: 3});
        assert_eq!(ops[2], StackOp{num: 2, from: 2, to: 1});
        assert_eq!(ops[3], StackOp{num: 1, from: 1, to: 2});
    }

    #[test]
    fn test_part2() {
        
    }
}
#[derive(PartialEq,Eq)]
#[derive(Debug)]
struct StackOp {
    num: i32,
    from: usize,
    to: usize,
}

fn parse_input(filepath: &str) -> (Vec<Vec<char>>, Vec<StackOp>) {
    let data: String = fs::read_to_string(filepath).expect("Cannot read file");
    let lines:Vec<&str> = data.split("\n").collect();
    let mut line_n = 0;

    let mut stacks:Vec<Vec<char>> = vec![];
    let mut ops:Vec<StackOp> = vec![];

    // Determine number of stacks
    let stacks_n = (lines[0].len() + 1)/ 4;
    for _ in 0..stacks_n {
        stacks.push(vec![]);
    }

    for line in &lines {
        line_n += 1;
        let mut pos = 0;
        if line.trim().chars().nth(0) == Some('1') {
            println!("Parsing done!");
            break;
        }
        for c in line.chars() {
            if c == ' ' || c == '[' || c == ']' {
                pos += 1;
                continue;
            }
            stacks[(pos - 1) / 4].push(c);
            pos += 1;
        }
    }

    for stack in &mut stacks {
        stack.reverse();
    }

    // Parse the StackOps now
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for line in &lines[line_n+1..] {
        if *line == "" {
            break;
        }

        let caps = re.captures(line).unwrap();
        let op = StackOp {
            num: caps.get(1).unwrap().as_str().parse().unwrap(),
            from: caps.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1,
            to: caps.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1,
        };
        ops.push(op);
    }
    
    (stacks, ops)

}

pub fn part1() {
    let (mut stacks, ops) = parse_input("./src/day05/input.txt");
    for op in ops {
        for _ in 0..op.num {
            let popped = stacks[op.from].pop().unwrap();
            stacks[op.to].push(popped);
        }
    }
    let result:String = stacks.iter().map(|x| x.last().unwrap()).collect();
    println!("ANS: {}", result);
}

pub fn part2() {
    let (mut stacks, ops) = parse_input("./src/day05/input.txt");
    for op in ops {
        // Popping as group
        let from_len = stacks[op.from].len();
        let num_remove = op.num as usize;
        //println!("from_len: {}\tnum_remove: {}", from_len, num_remove);
        let sliced:Vec<char> = Vec::from(&stacks[op.from][from_len-num_remove..from_len]);
        //println!("Sliced: {:?}",sliced);
        stacks[op.from].drain(from_len-num_remove..from_len);

        //Pushing the result
        stacks[op.to].extend(sliced);
    }
    let result:String = stacks.iter().map(|x| x.last().unwrap()).collect();
    println!("ANS: {}", result);
}