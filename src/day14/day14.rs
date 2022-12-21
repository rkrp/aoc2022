use std::{fs, fmt};
use std::{thread, time::Duration};

#[derive(PartialEq,Eq)]
#[derive(Debug)]
#[derive(Clone, Copy)]
enum State {
    FREE,
    ROCK,
    SAND,
}

#[derive(PartialEq,Eq)]
#[derive(Clone, Copy)]
struct Position {   
    row: i32,   // Need i32 here as invalid position can
    col: i32,   // be negative integers
}


impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.row, self.col)
    }
}

impl Position {
    // Swapping args here as the input in this stupid format
    fn new(col: i32, row:i32 ) -> Self { 
        Position { row, col }
    }

    fn from_str(inp: &str) -> Self {
        let foo:Vec<i32> = inp.split(",").map(|f| f.parse().unwrap()).collect();
        Position::new(foo[0], foo[1])
    }

    fn _get_north(&self) -> Position {
        Position::new(self.col, self.row - 1)
    }

    fn get_south(&self) -> Position {
        Position::new(self.col, self.row + 1)
    }

    fn get_east(&self) -> Position {
        Position::new(self.col + 1, self.row)
    }

    fn get_west(&self) -> Position {
        Position::new(self.col -1, self.row)
    }
}

const WIDTH: usize = 1000;
const HEIGHT: usize = 500;
const DRIPPER:Position = Position {row: 0, col: 500};

struct Plane {
    plane: [[State;WIDTH];HEIGHT],
    end_on_abyss: bool,
    animate: bool,
    abyss: usize,
}

impl Plane {
    fn new(end_on_abyss: bool, animate: bool) -> Self {
        let plane = [[State::FREE; WIDTH]; HEIGHT];
        let abyss = 0;
        Plane { plane, abyss, end_on_abyss, animate }
    }

    fn get_state(&self, position: Position) -> State {
        self.plane[position.row as usize][position.col as usize]
    }

    fn mark_sand(&mut self, position: Position) {
        self.plane[position.row as usize][position.col as usize] = State::SAND;
    }

    fn print(&self) {
        self.print_anchor(Position::new(480, 0), 40, 15);        
    }

    fn next_drip_move(&self, current: Position) -> Position {
        let south = current.get_south();
        if self.get_state(south) == State::FREE {
            return south;
        }

        let south_west = south.get_west();
        if self.get_state(south_west) == State::FREE {
            return south_west;
        }

        let south_east = south.get_east();
        if self.get_state(south_east) == State::FREE {
            return south_east;
        }

        // Return current position, if we cannot move anymore
        current
    }

    fn drip_one(&self) -> Option<Position> {
        let mut current = DRIPPER;
        loop {
            let next = self.next_drip_move(current);
            if next == current {
                return Some(current);
            }
            if self.end_on_abyss {
                if next.row >= self.abyss as i32 {
                    return None;
                }
            } 
            
            current = next;
        }
    }

    fn simulate_drips2(&mut self) -> i32 {
        let mut drip_counter = 0;

        // Construct the wall at the bottom
        let row = self.abyss + 2;
        for i in 0..WIDTH {
            self.plane[row][i] = State::ROCK;
        }

        loop {
            let rested = self.drip_one();
            match rested {
                None => { return drip_counter; }, 
                Some(x) => {
                    if x == DRIPPER {
                        return drip_counter;
                    }
                }
            };

            self.mark_sand(rested.unwrap());
            drip_counter += 1;

            if self.animate {
                print!("\x1B[2J");
                self.print();
                thread::sleep(Duration::from_millis(50));
            }
        }
    }

    fn simulate_drips(&mut self) -> i32 {
        let mut drip_counter = 0;
        loop {
            let rested = self.drip_one();
            match rested {
                None => { return drip_counter; }, 
                _ => {}
            };

            self.mark_sand(rested.unwrap());
            drip_counter += 1;

            if self.animate {
                print!("\x1B[2J");
                self.print();
                thread::sleep(Duration::from_millis(500));
            }
        }
    }

    fn print_anchor(&self, anchor: Position, width_offset:i32, height_offset:i32) {
        for row in anchor.row..(anchor.row+height_offset) {
            for col in anchor.col..(anchor.col+width_offset) {
                let chr = match self.plane[row as usize][col as usize] {
                    State::FREE => '.',
                    State::ROCK => '#',
                    State::SAND => 'O',
                };
                print!("{}", chr);
            }
            println!("");
        }
    }

    fn draw_wall(&mut self, positions: Vec<Position>) {
        for i in 0..positions.len()-1 {
            self.draw_line_wall(positions[i], positions[i+1]);

            // Check for new abyss level and set it
            if positions[i].row as usize > self.abyss {
                self.abyss = positions[i].row as usize;
            }

            if positions[i+1].row as usize > self.abyss {
                self.abyss = positions[i+1].row as usize;
            }
        }
    }

    fn draw_line_wall(&mut self, start: Position, end: Position) {
        if start.row == end.row {
            let iterrange = if start.col < end.col {
                start.col..=end.col
            } else {
                end.col..=start.col
            };

            for i in iterrange {
                self.plane[start.row as usize][i as usize] = State::ROCK;
            }
        } else if start.col == end.col {
            let iterrange = if start.row < end.row {
                start.row..=end.row
            } else {
                end.row..=start.row
            };

            for i in iterrange {
                self.plane[i as usize][start.col as usize] = State::ROCK;
            }
        } else {
            panic!("Invalid wall specs received!");
        }
    }

    

}


fn parse_input(filepath: &str, end_on_abyss: bool) -> Plane {
    let data: String = fs::read_to_string(filepath).expect("Cannot read file");
    let data = data.trim().lines();
    let data: Vec<Vec<_>> = data.map(|x| x.split(" -> ")
                        .map(|f| Position::from_str(f))
                        .collect())
                    .collect();

    let mut plane = Plane::new(end_on_abyss, false);
    for positions in data {
        plane.draw_wall(positions);
    }
    plane
}


pub fn part1() {
    // Get the walls 
    let mut plane = parse_input("./src/day14/input.txt", true);
    // Simulate sand dropping until one goes over the abyss
    let result = plane.simulate_drips();
    println!("Result: {}", result);
}

pub fn part2() {
    let mut plane = parse_input("./src/day14/input.txt", false);
    let result = plane.simulate_drips2();
    println!("ANS: {}", result + 1);
}

#[cfg(test)]
mod tests {

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    //use super::*;

    use super::*;

    fn assert_wall(plane: &Plane, wall: &Vec<Position>) {
        for row in 0..HEIGHT {
            for col in 0..WIDTH {
                let curr_pos = Position::new(col as i32, row as i32);
                if wall.contains(&curr_pos) {
                    assert_eq!(plane.plane[row][col], State::ROCK);    
                } else {
                    assert_eq!(plane.plane[row][col], State::FREE);
                }
            }
        }
    }

    #[test]
    fn draw_wall_test() {
        println!("Starting test..");
        let mut plane = Plane::new(true, true);
        println!("Does this even work?");
        let wall = vec![
            Position::new(498, 4),
            Position::new(498, 6),
            Position::new(496, 6)];
        plane.draw_wall(wall);

        let positions = [(498, 4), (498, 5), (498, 6), (497, 6), (496, 6)];
        let mut positions:Vec<Position> = positions.iter().map(|x| Position::new(x.0, x.1)).collect();
        assert_wall(&plane, &positions);
        
        let wall = vec![
            Position::new(503, 4),
            Position::new(502, 4),
            Position::new(502, 9),
            Position::new(494, 9)];
        plane.draw_wall(wall);

        let pos2 = [(503, 4), (502, 4), (502, 5), (502, 6), (502, 7), (502, 8), (502, 9), 
                                      (501, 9), (500, 9), (499, 9), (498, 9), (497, 9), (496, 9), (495, 9), (494, 9)];
        let pos2:Vec<Position> = pos2.iter().map(|x| Position::new(x.0, x.1)).collect();
        positions.extend(pos2);
        println!("{:?}", positions);
        plane.print();
        assert_wall(&plane, &positions);
        assert_eq!(plane.abyss, 9);

        let rested = plane.drip_one().unwrap();
        plane.mark_sand(rested);
        assert_eq!(rested, Position::new(500, 8));

        let rested = plane.drip_one().unwrap();
        plane.mark_sand(rested);
        assert_eq!(rested, Position::new(499, 8));

        let rested = plane.drip_one().unwrap();
        plane.mark_sand(rested);
        assert_eq!(rested, Position::new(501, 8));

        let rested = plane.drip_one().unwrap();
        plane.mark_sand(rested);
        assert_eq!(rested, Position::new(500, 7));

        let rested = plane.drip_one().unwrap();
        plane.mark_sand(rested);
        assert_eq!(rested, Position::new(498, 8));
    }

    #[test]
    fn full_simulate_test() {
        let mut plane = Plane::new(true, true);
        let wall = vec![
            Position::new(498, 4),
            Position::new(498, 6),
            Position::new(496, 6)];
        plane.draw_wall(wall);        
        
        let wall = vec![
            Position::new(503, 4),
            Position::new(502, 4),
            Position::new(502, 9),
            Position::new(494, 9)];
        plane.draw_wall(wall);

        assert_eq!(plane.abyss, 9);

        let drips = plane.simulate_drips();
        println!("DRIPS: {}", drips);
    }

    #[test]
    fn operation_test() {

    }
}