use std::{fs};

#[cfg(test)]
mod tests {

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    //use super::*;

    use crate::day08::day08::{Forest, parse_input};

    #[test]
    fn test_count_visible_trees() {         
        let forest:Vec<Vec<u8>> = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];        

        let f = Forest {forest: forest, height: 5, width: 5};
        assert_eq!(f.is_visible(1, 1), true);
        assert_eq!(f.is_visible(1, 2), true);
        assert_eq!(f.is_visible(1, 3), false);
        assert_eq!(f.is_visible(2, 1), true);
        assert_eq!(f.is_visible(2, 2), false);
        assert_eq!(f.is_visible(2, 3), true);
        assert_eq!(f.is_visible(3, 1), false);
        assert_eq!(f.is_visible(3, 2), true);
        assert_eq!(f.is_visible(3, 3), false);
        let result = f.solve_part1();
        println!("TADA {}", result);
    }


    #[test]
    fn test_input_parser() {
        let forest:Vec<Vec<u8>> = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];

        let foo = parse_input("./src/day08/sample_input.txt");

        let f = Forest {forest: forest, height: 5, width: 5};
        for i in 0..5 {
            assert_eq!(foo.forest[i], f.forest[i]);
        }
        assert_eq!(f.height, foo.height);
        assert_eq!(f.width, foo.width);
    }

    #[test]
    fn test_9_92() {
        let forest = parse_input("./src/day08/input.txt");
        let result = forest.is_visible(9, 92);
        assert_eq!(result, false);
    }
}

#[derive(PartialEq,Eq)]
#[derive(Debug)]
struct Forest {
    forest: Vec<Vec<u8>>,
    height: usize,
    width: usize,
}

impl Forest {
    fn is_viewable_right(&self, row: usize, col: usize) -> bool {
        let me = self.forest[row][col];
        for j in col+1..self.width {
            if self.forest[row][j] >= me {
                return false;
            }
        }
        true
    }

    fn is_viewable_left(&self, row: usize, col: usize) -> bool {
        let me = self.forest[row][col];
        for j in 0..col {
            if self.forest[row][j] >= me {
                return false;
            }
        }
        true
    }

    fn is_viewable_down(&self, row: usize, col: usize) -> bool {
        let me = self.forest[row][col];
        for i in row+1..self.height {
            if self.forest[i][col] >= me {
                return false;
            }
        }
        true
    }

    fn is_viewable_up(&self, row: usize, col: usize) -> bool {
        let me = self.forest[row][col];
        for i in 0..row {
            if self.forest[i][col] >= me {
                return false;
            }
        }
        true
    }

    fn is_visible(&self, row: usize, col: usize) -> bool {
        let left = self.is_viewable_left(row, col);
        let right = self.is_viewable_right(row, col);
        let up = self.is_viewable_up(row, col);
        let down = self.is_viewable_down(row, col);
        
        left || right || up || down
    }

    fn solve_part1(&self) -> i32 {
        let mut result = 0;
        let height = self.forest.len();
        let width = self.forest[0].len();
        
        // Add all the trees in perimeter
        result += 2 * height + 2 * width - 4;

        for row in 1..height-1{
            for col in 1..width-1 {
                if self.is_visible(row, col) {
                    //println!("[{}][{}] = {}", row, col, self.forest[row][col]);
                    result += 1;
                }
            }
        }
        result as i32
    }
}

fn parse_input(filepath: &str) -> Forest {
    let data: String = fs::read_to_string(filepath).expect("Cannot read file");
    let data:Vec<_> = data.trim().split("\n").collect();
    let data:Vec<Vec<_>> = data.iter()
                                .map(|x| x.chars()
                                    .map(|y| y.to_digit(10).unwrap() as u8)
                                    .collect())
                                .collect();
    Forest {height: data.len(), width: data[0].len(), forest: data, }

}

pub fn part1() {
    //let f = parse_input("./src/day08/sample_input.txt");
    let f = parse_input("./src/day08/input.txt");
    let result = f.solve_part1();
    println!("ANS: {:?}", result);
}

pub fn part2() {
    let fs = parse_input("./src/day08/sample_input.txt");
    let mut result = 0;

    println!("ANS: {:?}", result);
}