use std::fs;

#[derive(PartialEq,Eq)]
#[derive(Debug)]
#[derive(Clone)]
enum Op {
    ADD,
    MUL,
}

#[derive(PartialEq,Eq)]
#[derive(Debug)]
#[derive(Clone)]
struct MonkeyDecider {
    divisibleby: u64,
    iftrue: u64,
    iffalse: u64,
}

impl MonkeyDecider {
    fn decide(&self, value: u64) -> bool {
        value % (self.divisibleby as u64) == 0
    }
}

#[derive(PartialEq,Eq)]
#[derive(Debug)]
#[derive(Clone)]
struct Operation {
    operator: Op,
    operand1: String,
    operand2: String,
}

impl Operation {
    fn resolve_operand(input: &str, old: u64) -> u64 {
        match input.parse::<u64>() {
            Ok(x) => x,
            Err(_e) => old,
        }
    }

    fn execute(&self, old: u64) -> u64 {
        let op1 = Operation::resolve_operand(&self.operand1, old);
        let op2 = Operation::resolve_operand(&self.operand2, old);
        match self.operator {
            Op::ADD => {
                op1 + op2
            },
            Op::MUL => {
                op1 * op2
            }
        }
    }

    fn new(input:&str) -> Self {
        let parsed: Vec<_> = input.split(" ").collect();
        let op = match parsed[1] {
            "+" => Op::ADD,
            "*" => Op::MUL,
            _ => panic!("Unexpected operator {}", parsed[1])
        };
        Operation{
            operand1: String::from(parsed[0]),
            operand2: String::from(parsed[2]),
            operator: op,
        }
    }

}

#[derive(PartialEq,Eq)]
#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    decider: MonkeyDecider,
    items_inspected: u64,
    div3: bool,
}

impl Monkey {
    fn new(items: Vec<u64>, operation: Operation, decider: MonkeyDecider, div3: bool) -> Self {
        let items_inspected = 0;
        Monkey { items, operation, decider, items_inspected, div3 }
    }

    fn do_monkey_stuff(&mut self) -> (Vec<u64>, Vec<u64>) {
        let mut true_vals:Vec<u64> = vec![];
        let mut false_vals:Vec<u64> = vec![];
        self.items_inspected += self.items.len() as u64;

        for i in 0..self.items.len() {
            let mut wlevel = self.items[i];
            if self.div3 {
                wlevel = self.operation.execute(wlevel) / 3;
            } else {
                wlevel = self.operation.execute(wlevel);
            }
            if self.decider.decide(wlevel) {
                true_vals.push(wlevel);
            } else {
                false_vals.push(wlevel);
            }
            
        }
        self.items = vec![];
        (true_vals, false_vals)
    }

}

enum ParseState {
    MonkeyIndex,
    Items,
    Operation,
    DivBy,
    IfTrue,
    IfFalse,
    End,
}

fn parse_input(filepath: &str, div3: bool) -> Vec<Monkey> {
    let data: String = fs::read_to_string(filepath).expect("Cannot read file");
    let data:Vec<_> = data.trim().split("\n").collect();

    let mut state = ParseState::MonkeyIndex;

    let mut monkeys: Vec<Monkey> = vec![];
    
    let mut items: Vec<u64> = vec![];
    let mut operation:Operation = Operation::new("1 * 2");
    let mut divby: u64 = 0;
    let mut iftrue: u64 = 0;
    for line in data {
        match state {
            ParseState::MonkeyIndex => {
                state = ParseState::Items;
            },
            ParseState::Items => {
                let (_, items_str) = line.rsplit_once(": ").unwrap();
                items = items_str.split(", ").map(|x| x.parse::<u64>().unwrap()).collect();
                state = ParseState::Operation;
            },
            ParseState::Operation => {
                let (_, operation_str) = line.rsplit_once("= ").unwrap();
                operation = Operation::new(operation_str);
                state = ParseState::DivBy;
            },
            ParseState::DivBy => {
                let (_, divby_str) = line.rsplit_once(" ").unwrap();
                divby = divby_str.parse::<u64>().unwrap();
                state = ParseState::IfTrue;
            },
            ParseState::IfTrue => {
                let (_, iftrue_str) = line.rsplit_once(" ").unwrap();
                iftrue = iftrue_str.parse::<u64>().unwrap();
                state = ParseState::IfFalse;
            },
            ParseState::IfFalse => {
                let (_, iftrue_str) = line.rsplit_once(" ").unwrap();
                let iffalse = iftrue_str.parse::<u64>().unwrap();
                state = ParseState::End;
                
                let decider = MonkeyDecider { divisibleby: divby, iftrue: iftrue, iffalse: iffalse };
                let monkey = Monkey::new(items.clone(), operation.clone(), decider, div3);
                monkeys.push(monkey);
            },
            ParseState::End => {
                state = ParseState::MonkeyIndex;
            }
        }
        
    }
    monkeys
}

fn do_monkey_business(monkeys: Vec<Monkey>, rounds:u64) -> u64 {
    let mut monkeys = monkeys;
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let (truevals, falsevals) = monkeys[i].do_monkey_stuff();
            let iftrue = monkeys[i].decider.iftrue as usize;
            let iffalse = monkeys[i].decider.iffalse as usize;
            monkeys[iftrue].items.extend(truevals);
            monkeys[iffalse].items.extend(falsevals);

        }
    }

    monkeys.sort_by_key(|x| x.items_inspected);
    monkeys.reverse();
    let result = monkeys[0].items_inspected * monkeys[1].items_inspected;
    result
}

pub fn part1() {
    let monkeys = parse_input("./src/day11/input.txt", true);
    let result = do_monkey_business(monkeys, 20);
    println!("ANS: {}", result);
    
}

pub fn part2() {
    let monkeys = parse_input("./src/day11/sample_input.txt", false);
    let result = do_monkey_business(monkeys, 20);
    println!("ANS: {}", result);
}

#[cfg(test)]
mod tests {

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    //use super::*;

    use super::*;

    #[test]
    fn test_part1() {
        let mut monkey0 = Monkey::new(
            vec![79, 98], 
            Operation::new("old * 19"),
            MonkeyDecider { divisibleby: 23, iftrue: 2, iffalse: 3 },
            true,
        );
        let mut monkey1 = Monkey::new(
            vec![54,65,75,74], 
            Operation::new("old + 6"),
            MonkeyDecider { divisibleby: 19, iftrue: 2, iffalse: 0 },
            true,
        );
        let mut monkey2 = Monkey::new(
            vec![79, 60, 97], 
            Operation::new("old * old"),
            MonkeyDecider { divisibleby: 13, iftrue: 1, iffalse: 3 },
            true,
        );
        let mut monkey3 = Monkey::new(
            vec![74], 
            Operation::new("old + 3"),
            MonkeyDecider { divisibleby: 17, iftrue: 0, iffalse: 1 },
            true,
        );

        let mut monkeys = vec![monkey0, monkey1, monkey2, monkey3];
        let result = do_monkey_business(monkeys, 20);
        assert_eq!(result, 10605);
    }

    #[test]
    fn test_part2() {
        let mut monkeys = parse_input("./src/day11/sample_input.txt", false);
        let result = do_monkey_business(monkeys, 10000);
        assert_eq!(result, 2713310158);
    }

    #[test]
    fn operation_test() {
        let op = Operation::new("old * 3");
        assert_eq!(op.execute(5), 15);

        let op = Operation::new("old + 4");
        assert_eq!(op.execute(6), 10);
        
        let op = Operation::new("old * old");
        assert_eq!(op.execute(9), 81);
    }
}