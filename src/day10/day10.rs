use std::fs;

#[derive(PartialEq,Eq)]
#[derive(Debug)]
enum Op {
    NOOP,
    ADDX,
}

#[derive(PartialEq,Eq)]
#[derive(Debug)]
struct OpCode {
    operator: Op,
    operand: i32,
}

impl OpCode {
    fn new(operator: Op, operand: i32) -> Self {
        OpCode{operator, operand}
    }
}

fn parse_input(filepath: &str) -> Vec<OpCode> {
    let data: String = fs::read_to_string(filepath).expect("Cannot read file");
    let data:Vec<_> = data.trim().split("\n").collect();
    let mut instructions = vec![];
    for line in data {
        if line == "noop" {
            instructions.push(OpCode::new(Op::NOOP, 0));
        } else {
            let opcode:Vec<_> = line.split(" ").collect();
            let operand:i32 = match opcode[1].parse() {
                Ok(x) => x,
                Err(_e) => panic!("Error reading {} as i32", opcode[1]),
            };
            instructions.push(OpCode::new(Op::ADDX, operand));
        }
    }
    instructions
}

fn calc_strength(cycles: i32, x: i32) -> i32 {
    let spl_cycles = Vec::from_iter((20..=220).step_by(40));
    if spl_cycles.contains(&cycles) {
        println!("Cycles: {}\tX: {}", cycles, x);
        return x * cycles;
    }
    0
}

fn draw_pixel(cycles:i32, x:i32) {
    let mut pixel = ' ';
    if ((cycles -1) % 40) >= (x - 1) && ((cycles -1) % 40) <= (x + 1) {
        pixel = '#'
    }
    
    
    
    if (cycles - 1 ) % 40 == 0 {
        print!("\n")
    }
    print!("{}", pixel);
}

fn execute(instructions: Vec<OpCode>) -> i32 {
    let mut cycles = 0;
    let mut x = 1;
    let mut result = 0;
    for opcode in instructions {
        match opcode.operator {
            Op::NOOP => {
                cycles += 1;
                result += calc_strength(cycles, x);
                draw_pixel(cycles, x);
            },
            Op::ADDX => {
                for _ in 0..2 {
                    cycles += 1;
                    result += calc_strength(cycles, x);
                    draw_pixel(cycles, x);
                }
                x += opcode.operand;
            }
        }
        //println!("DEBUG: cycles: {} X: {}", cycles, x);
    }
    result
}

pub fn part1() {
    let instructions = parse_input("./src/day10/input.txt");
    let result = execute(instructions);
    println!("ANS: {:?}", result);
}

pub fn part2() {
    // let fs = parse_input("./src/day09/input.txt");
    // let result = fs.solve_part2();
    // println!("ANS: {:?}", result);
}

#[cfg(test)]
mod tests {

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    //use super::*;

    use super::*;

    #[test]
    fn test_sample_input() {
        let instructions = vec![
            OpCode::new(Op::NOOP, 0),
            OpCode::new(Op::ADDX, 3),
            OpCode::new(Op::ADDX, -5),
        ];
        execute(instructions);
    }
}