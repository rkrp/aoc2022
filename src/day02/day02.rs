use std::fs;

fn get_score(move1: i32, move2: i32) -> i32 {
    // Draw scenario
    if move1 == move2 {
        return 3 + move1 + 1;
    }

    // Win scenario
    if (move1 + 1) % 3 == move2 {
        return 6 + move2 + 1;
    } 

    // Lose scenario
    if (move2 + 1) % 3 == move1 {
        return move2 + 1;
    }
    
    panic!("This should not be reached!");
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_get_score_p1() {
        assert_eq!(get_score(0, 0), 3+1);
        assert_eq!(get_score(0, 1), 6+2);
        assert_eq!(get_score(0, 2), 0+3);
        assert_eq!(get_score(1, 0), 0+1);
        assert_eq!(get_score(1, 1), 3+2);
        assert_eq!(get_score(1, 2), 6+3);
        assert_eq!(get_score(2, 0), 6+1);
        assert_eq!(get_score(2, 1), 0+2);
        assert_eq!(get_score(2, 2), 3+3);
    }
}


fn get_move_int(inp: char) -> i32 {
    match inp {
        'A' => 0,
        'B' => 1,
        'C' => 2,
        'X' => 0,
        'Y' => 1,
        'Z' => 2,
        _ => panic!("Unexpected char {}", inp),
    }
}

pub fn part1() {
    let data: String = fs::read_to_string("./src/day02/input.txt").expect("Cannot read file");
    let mut score :i32= 0;
    //println!("{}", data);

    let lines:Vec<&str> = data.trim().split("\n").collect();
    for line in lines {
        let moves:Vec<&str>= line.trim().split(" ").collect();
        let moves:Vec<char> = moves.iter().map(|x| x.chars().nth(0).unwrap()).collect();
        let move1 = get_move_int(moves[0]);
        let move2 = get_move_int(moves[1]);
        score += get_score(move1, move2);

    }
    println!("ANS: {}", score);
}

fn get_score_part2(opp_move: i32, outcome: char) -> i32 {
    if opp_move > 2 {
        panic!("Invalid opp_move");
    }

    match outcome {
        'X' => {
            // Lose scenario
            let mymove = (opp_move - 1 + 3) % 3;
            let score = mymove + 1;
            score
        },
        'Y' => {
            // Draw scenario
            let mymove = opp_move;
            let score = mymove + 1 + 3;
            score
        },
        'Z' => {
            // Win scenario
            let mymove = (opp_move + 1) % 3;
            let score = mymove + 1 + 6;
            score
        }
        _ => panic!("Invalid outcome")
    }    
}

pub fn part2() {
    let data: String = fs::read_to_string("./src/day02/input.txt").expect("Cannot read file");
    let mut total :i32= 0;

    let lines:Vec<&str> = data.trim().split("\n").collect();
    for line in lines {
        let moves:Vec<&str>= line.trim().split(" ").collect();
        let moves:Vec<char> = moves.iter().map(|x| x.chars().nth(0).unwrap()).collect();
        let opp_move = get_move_int(moves[0]);
        let outcome = moves[1];
        let score = get_score_part2(opp_move, outcome);
        println!("{}", score);
        if score == 0 {
            break;
        }

        total += score;
    }
    println!("ANS: {}", total);
}