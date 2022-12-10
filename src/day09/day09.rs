use std::{fs, collections::HashMap};

#[derive(Copy, Clone)]
#[derive(PartialEq,Eq)]
#[derive(Debug)]
enum Direction {
    West,
    East,
    North,
    South,
}

#[derive(Copy, Clone)]
#[derive(PartialEq,Eq)]
#[derive(Debug)]
enum RelativePosition {
    West,
    East,
    North,
    South,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Overlapping,
    NoPos,
}

#[derive(Hash)]
#[derive(Copy, Clone)]
#[derive(PartialEq,Eq)]
#[derive(Debug)]
struct Position {
    row: i32,
    col: i32,
}

#[derive(PartialEq,Eq)]
#[derive(Debug)]
struct Instruction {
    direction: Direction,
    distance: u32,
}

impl Instruction {
    fn new(direction: Direction, distance: u32) -> Self {
        Instruction {direction, distance}
    }
}


impl Position {
    fn new(row:i32, col:i32) -> Self {
        Position { row, col }    
    }

    fn move_one(&mut self, direction: Direction ) {
        match direction {
            Direction::East => {
                self.move_east();
            },
            Direction::West => {
                self.move_west();
            },
            Direction::North => {
                self.move_north();
            },
            Direction::South => {
                self.move_south();
            },
        }
    }

    fn move_north(&mut self) {
        self.row -= 1;
    }

    fn move_south(&mut self) {
        self.row += 1;
    }

    fn move_east(&mut self) {
        self.col += 1;
    }

    fn move_west(&mut self) {
        self.col -= 1;
    }

}

#[derive(PartialEq,Eq)]
#[derive(Debug)]
struct PlayGround {
    // Need negative int here as the rope 
    // can travel in all 4 quadrants
    head: Position,
    tail: Position,
    counter: HashMap<Position, bool>,
}

impl PlayGround {
    fn new() -> Self {
        let head = Position::new(0, 0);
        let tail = Position::new(0, 0);
        let mut counter = HashMap::new(); 
        counter.insert(tail, true);
        PlayGround { head, tail, counter }
    }

    // Gets the relative position of the tail with respect to 
    // the current head position
    fn get_relative_position(&self) -> RelativePosition {
        if self.head == self.tail {
            return RelativePosition::Overlapping;
        }

        if self.head.row == self.tail.row && self.tail.col + 1 == self.head.col {
                return RelativePosition::West;
        }
        if self.head.row == self.tail.row && self.tail.col - 1 == self.head.col {
                return RelativePosition::East;
        }                                                                        

        if self.head.col == self.tail.col && self.tail.row + 1 == self.head.row {
                return RelativePosition::North;
        }
        if self.head.col == self.tail.col && self.tail.row - 1 == self.head.row {
                return RelativePosition::South;
        }

        // Cross positions
        if self.tail.row + 1 == self.head.row && self.tail.col + 1 == self.head.col {
            return RelativePosition::NorthWest;
        }

        if self.tail.row + 1 == self.head.row && self.tail.col - 1 == self.head.col {
            return RelativePosition::NorthEast;
        }

        if self.tail.row - 1 == self.head.row && self.tail.col + 1 == self.head.col {
            return RelativePosition::SouthWest;
        }

        if self.tail.row - 1 == self.head.row && self.tail.col - 1 == self.head.col {
            return RelativePosition::SouthEast;
        }


        return RelativePosition::NoPos;
    } 

    fn execute_instruction(&mut self, instruction:Instruction) {
        let mut head_old;
        for _ in 0..instruction.distance {
            head_old = self.head;
            self.head.move_one(instruction.direction);
            if self.get_relative_position() == RelativePosition::NoPos {
                self.tail = head_old;
                self.counter.insert(self.tail, true);
            }
        }
    }

    fn execute(&mut self, instructions:Vec<Instruction>) {
        for op in instructions {
            self.execute_instruction(op);
        }
    }
}

fn parse_input(filepath: &str) -> Vec<Instruction> {
    let data: String = fs::read_to_string(filepath).expect("Cannot read file");
    let data:Vec<_> = data.trim().split("\n").collect();
    let mut instructions = vec![];
    for line in data {
        let op:Vec<_> = line.split(" ").collect();
        let direction = match op[0] {
            "R" => Direction::East,
            "L" => Direction::West,
            "U" => Direction::North,
            "D" => Direction::South,
            _ => panic!("Unexpected Direction string: {}", op[0])
        };
        let distance:u32 = op[1].parse().unwrap();
        instructions.push(Instruction::new(direction, distance));
    }
    instructions
}

pub fn part1() {
    let instructions = parse_input("./src/day09/input.txt");
    let mut playground = PlayGround::new();
    playground.execute(instructions);
    let result = playground.counter.len();

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
            Instruction::new(Direction::East, 4),
            Instruction::new(Direction::North, 4),
            Instruction::new(Direction::West, 3),
            Instruction::new(Direction::South, 1),
            Instruction::new(Direction::East, 4),
            Instruction::new(Direction::South, 1),
            Instruction::new(Direction::West, 5),
            Instruction::new(Direction::East, 2),
        ];
        let mut playground = PlayGround::new();
        playground.execute(instructions);

        assert_eq!(Position::new(-2, 1), playground.tail);
        assert_eq!(Position::new(-2, 2), playground.head);

        let tail_count = playground.counter.len();
        assert_eq!(tail_count, 13);

    }

    #[test]
    fn exec_one_instruction() {
        let mut playground = PlayGround::new();
        
        playground.execute_instruction(Instruction::new(Direction::East, 4));
        assert_eq!(playground.head, Position::new(0, 4));
        assert_eq!(playground.tail, Position::new(0, 3));

        playground.execute_instruction(Instruction::new(Direction::North, 4));
        assert_eq!(playground.head, Position::new(-4, 4));
        assert_eq!(playground.tail, Position::new(-3, 4));
    }

    #[test]
    fn relative_postitions_test() {        
        let playground = PlayGround {tail: Position::new(1, 0), head: Position::new(1, 1), counter: HashMap::new()};
        assert_eq!(playground.get_relative_position(), RelativePosition::West);

        let playground = PlayGround {tail: Position::new(1, 2), head: Position::new(1, 1), counter: HashMap::new()};
        assert_eq!(playground.get_relative_position(), RelativePosition::East);

        let playground = PlayGround {tail: Position::new(0, 1), head: Position::new(1, 1), counter: HashMap::new()};
        assert_eq!(playground.get_relative_position(), RelativePosition::North);

        let playground = PlayGround {tail: Position::new(2, 1), head: Position::new(1, 1), counter: HashMap::new()};
        assert_eq!(playground.get_relative_position(), RelativePosition::South);

        let playground = PlayGround {tail: Position::new(0, 2), head: Position::new(1, 1), counter: HashMap::new()};
        assert_eq!(playground.get_relative_position(), RelativePosition::NorthEast);

        let playground = PlayGround {tail: Position::new(0, 0), head: Position::new(1, 1), counter: HashMap::new()};
        assert_eq!(playground.get_relative_position(), RelativePosition::NorthWest);

        let playground = PlayGround {tail: Position::new(2, 2), head: Position::new(1, 1), counter: HashMap::new()};
        assert_eq!(playground.get_relative_position(), RelativePosition::SouthEast);

        let playground = PlayGround {tail: Position::new(2, 0), head: Position::new(1, 1), counter: HashMap::new()};
        assert_eq!(playground.get_relative_position(), RelativePosition::SouthWest);

        let playground = PlayGround {tail: Position::new(1, 1), head: Position::new(1, 1), counter: HashMap::new()};
        assert_eq!(playground.get_relative_position(), RelativePosition::Overlapping);
    }
}