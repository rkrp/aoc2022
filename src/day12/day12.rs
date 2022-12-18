use std::{fs, fmt, collections::{HashMap, HashSet}};
use petgraph::{Graph, stable_graph::NodeIndex, visit::{IntoNodeReferences, IntoEdgesDirected}, Direction::{Incoming, Outgoing}, graph::Node};
use petgraph::dot::{Dot, Config};

#[derive(Debug)]
struct Climb; 

#[derive(Debug)]
struct Forest {
    forest: Vec<Vec<u8>>,
    height: usize,
    width: usize,
    start: Position,
    end: Position,
    graph: Graph<Position, Climb>,
    nodeindices: Vec<Vec<NodeIndex>>,
    counter: HashSet<char>
}

enum Direction {
    West,
    East,
    North,
    South,
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
    fn new(row: i32, col:i32 ) -> Self {
        Position { row, col }
    }

    fn get_position(&self, direction: Direction ) -> Position {
        match direction {
            Direction::East => {
                self.get_east()
            },
            Direction::West => {
                self.get_west()
            },
            Direction::North => {
                self.get_north()
            },
            Direction::South => {
                self.get_south()
            },
        }
    }

    fn get_north(&self) -> Position {
        Position::new(self.row - 1, self.col)
    }

    fn get_south(&self) -> Position {
        Position::new(self.row + 1, self.col)
    }

    fn get_east(&self) -> Position {
        Position::new(self.row, self.col + 1)
    }

    fn get_west(&self) -> Position {
        Position::new(self.row, self.col -1)
    }

}

impl Forest {
    fn new(height: usize, width: usize, forest: Vec<Vec<u8>>) -> Self {
        let mut nodeindices:Vec<Vec<NodeIndex>> = vec![];
        let mut graph:Graph<Position, Climb> = Graph::new();
        let counter: HashSet<char> = HashSet::new();
        
        for row in 0..height {
            nodeindices.push(vec![]);
            for col in 0..width {
                let nodeindex = graph.add_node(Position::new(row as i32, col as i32));
                nodeindices[row].push(nodeindex);
            }
        }

        Forest {
            height,
            width,
            forest, 
            start: Position::new(0, 0),
            end: Position::new(0, 0),
            graph,
            nodeindices,
            counter,
        }
    }
    
    fn get_start(&self) -> Position {
        for row in 0..self.height {
            for col in 0..self.width {
                if self.forest[row][col] == 'S' as u8 {
                    return Position::new(row as i32, col as i32);
                }
            }
        }
        panic!("Start not present in input");
    }

    fn get_end(&self) -> Position {
        for row in 0..self.height {
            for col in 0..self.width {
                if self.forest[row][col] == 'E' as u8 {
                    return Position::new(row as i32, col as i32);
                }
            }
        }
        panic!("End not present in input");
    }

    fn init_start_end(&mut self) {
        self.start = self.get_start();
        self.end = self.get_end();
        self.set_forest_value(self.start, 'a' as u8 - 1);
        self.set_forest_value(self.end, 'z' as u8 + 1);
    }

    fn set_forest_value(&mut self, pos: Position, value: u8) {
        self.forest[pos.row as usize][pos.col as usize] = value;

    }

    fn get_forest_value(&self, pos: Position) -> Option<&u8> {
        if pos.row < 0 || pos.col < 0 {
            return None;
        }

        match self.forest.get(pos.row as usize) {
            Some(x) => {
                match x.get(pos.col as usize) {
                    Some(val) => { Some(val) },
                    None => {None}
                }
            }
            None => {None}
        }
    }

    fn get_nodeindex(&self, pos: Position) -> NodeIndex {
        self.nodeindices[pos.row as usize][pos.col as usize]
    }

    fn gen_graph(&mut self, pos: Position) {        
        self.counter.insert(*(self.get_forest_value(pos)).unwrap() as char);
        
        if self.graph.edges_directed(self.get_nodeindex(pos), Outgoing).count() != 0 {
            // We are looping because this was already visited
            return;
        }
        
        let moves = self.get_possible_moves(pos);
        if moves.is_empty() {
            return;
        }        
        
        let startindex = self.get_nodeindex(pos);
        for mov in moves {
            // Add edge connecting the possible climb
            let endindex = self.get_nodeindex(mov);
            self.graph.add_edge(startindex, endindex, Climb);
            self.gen_graph(mov);
        }
    }

    fn get_possible_moves(&self, pos: Position) -> Vec<Position> {
        let mut moves = vec![];
        let directions = vec![
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ];
        let current_val = self.get_forest_value(pos).unwrap();
        for direction in directions {
            let new_pos = pos.get_position(direction);
            match self.get_forest_value(new_pos) {
                Some(x) => { 
                    if current_val + 1 == *x || current_val >= x{
                        moves.push(new_pos)
                    }
                },
                None => {}
            }
        }
        moves
    }
}


fn parse_input(filepath: &str) -> Forest {
    let data: String = fs::read_to_string(filepath).expect("Cannot read file");
    let data:Vec<Vec<_>> = data.trim().lines()
                                .map(|x| x.chars()
                                    .map(|y| y as u8)
                                    .collect())
                                .collect();
    let mut forest = Forest::new(data.len(), data[0].len(), data);
    forest.init_start_end();
    //forest.init_nodes();
    forest.gen_graph(forest.start);
    forest
}

pub fn part1() {
    let forest = parse_input("./src/day12/input.txt");
    let dot = Dot::with_config(&forest.graph, &[Config::EdgeNoLabel]);
    //println!("{:?}", dot);
    let path = petgraph::algo::astar(&forest.graph, 
        forest.get_nodeindex(forest.start),
        |finish| finish == forest.get_nodeindex(forest.end),
        |_| 1,
        |_| 1);
    match path {
        Some((_, path)) => {
            println!("Pathlen: {}", path.len() - 1);
            // let path:Vec<Position> = path.iter().map(|x| forest.graph[*x]).collect();
            // let vals:Vec<char> = path.iter().map(|x| *(forest.get_forest_value(*x).unwrap()) as char).collect();
            // println!("PATH: {:?}", path);
            // println!("PATH: {:?}", vals);
        },
        None => {
            println!("Path not found!");
        }
    };
    //println!("Counter: {:?}", forest.counter);
    //println!("ANS: {}", result);

    
}

pub fn part2() {
    let mut forest = parse_input("./src/day12/input.txt");
    let dot = Dot::with_config(&forest.graph, &[Config::EdgeNoLabel]);
    //println!("{:?}", dot);

    let mut start_pos: Vec<Position> = vec![];
    for row in 0..forest.height {
        for col in 0..forest.width {
            if forest.forest[row][col] == 'a' as u8 -1 || 
                forest.forest[row][col] == 'a' as u8
            {
                start_pos.push(Position::new(row as i32, col as i32));
            }
        }
    }
    println!("Start positions: {:?}", start_pos);
    let mut min = usize::MAX;
    for start in start_pos {
        println!("Chekcing path for {:?}", start);
        // Get rid of all edges 
        forest.graph.clear_edges();
        forest.counter = HashSet::new();
        // Generate graph again with a new start point
        forest.gen_graph(start);
        
        
        let path = petgraph::algo::astar(&forest.graph, 
            forest.get_nodeindex(start),
            |finish| finish == forest.get_nodeindex(forest.end),
            |_| 1,
            |_| 1);
        match path {
            Some((_, path)) => {
                let result = path.len() - 1;
                if result < min {
                    min = result;
                }
            },
            None => {
                println!("Path not found for {:?}", start);
                //let dot = Dot::with_config(&forest.graph, &[Config::EdgeNoLabel]);
                //println!("{:?}", dot);
                //panic!("Counter: {:?}", forest.counter);
            }
        };
    }
    println!("ANS: {}", min);
}

#[cfg(test)]
mod tests {

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    //use super::*;

    use super::*;

    #[test]
    fn test_parser() {
        let forest = parse_input("./src/day12/sample_input.txt");
        let moves = forest.get_possible_moves(forest.start);
        println!("{:?}", moves);
    }

    #[test]
    fn test_part2() {

    }

}