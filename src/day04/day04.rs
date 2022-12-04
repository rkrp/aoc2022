use std::fs;

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_get_intersection() {
        let foo: &str = &"12-34"[..];
        let a:Vec<i32> = foo.split("-").map(|x| x.parse::<i32>().unwrap()).collect();
        println!("{:?}", a);
    }

    #[test]
    fn test_part1() {
        assert_eq!(is_within_range(&Entry{start: 2, end: 4}, &Entry{start:6, end: 8}), false);
        assert_eq!(is_within_range(&Entry{start: 2, end: 3}, &Entry{start:4, end: 5}), false);
        assert_eq!(is_within_range(&Entry{start: 5, end: 7}, &Entry{start:7, end: 9}), false);
        assert_eq!(is_within_range(&Entry{start: 2, end: 8}, &Entry{start:3, end: 7}), true);
        assert_eq!(is_within_range(&Entry{start: 6, end: 6}, &Entry{start:4, end: 6}), true);
        assert_eq!(is_within_range(&Entry{start: 2, end: 6}, &Entry{start:4, end: 8}), false);
    }

    #[test]
    fn test_part2() {
        assert_eq!(is_overlapping(Entry{start: 2, end: 4}, Entry{start:6, end: 8}), false);
        assert_eq!(is_overlapping(Entry{start: 2, end: 3}, Entry{start:4, end: 5}), false);
        assert_eq!(is_overlapping(Entry{start: 5, end: 7}, Entry{start:7, end: 9}), true);
        assert_eq!(is_overlapping(Entry{start: 2, end: 8}, Entry{start:3, end: 7}), true);
        assert_eq!(is_overlapping(Entry{start: 6, end: 6}, Entry{start:4, end: 6}), true);
        assert_eq!(is_overlapping(Entry{start: 2, end: 6}, Entry{start:4, end: 8}), true);
    }
}


#[derive(Debug)]
struct Entry {
    start: i32,
    end: i32,
}

fn is_overlapping(entry1: Entry, entry2: Entry) -> bool {
    /*if is_within_range(&entry1, &entry2) {
        return true;
    }*/

    // Check if [2,6] [4,8] that 4 is present within [2,6]
    if entry2.start >= entry1.start && entry2.start <= entry1.end {
        return true;
    }

    // Swap and check the above 
    if entry1.start >= entry2.start && entry1.start <= entry2.end {
        return true;
    }
    false
}

fn is_within_range(entry1: &Entry, entry2: &Entry) -> bool {
    // Check if entry1 is big enough to contain entry2
    if entry1.start <= entry2.start && entry1.end >= entry2.end {
        return true;
    }

    // Check if entry2 is big enough to contain entry1
    if entry2.start <= entry1.start && entry2.end >= entry1.end {
        return true;
    }

    false
}

pub fn part1() {
    let data: String = fs::read_to_string("./src/day04/input.txt").expect("Cannot read file");
    let mut score = 0;
    let lines:Vec<&str> = data.trim().split("\n").collect();
    for line in lines {
        //let parsed:Vec<Vec<i32>> = 
        let parsed:Vec<Vec<i32>> = line.split(",")
                .map(|x| x.split("-").map(|y| y.parse::<i32>().unwrap()).collect())
                .collect();

        let entry1 = Entry{
            start: parsed[0][0],
            end: parsed[0][1],
        };

        let entry2 = Entry{
            start: parsed[1][0],
            end: parsed[1][1],
        };
        //println!("Checking {:?} and {:?}", entry1, entry2);
        if is_within_range(&entry1, &entry2) {
            //println!("Yah!");
            score += 1;
        } else {
            //println!("Nah!");
        }
        
    }
    println!("ANS: {}", score);
}

pub fn part2() {
    let data: String = fs::read_to_string("./src/day04/input.txt").expect("Cannot read file");
    let mut score = 0;
    let lines:Vec<&str> = data.trim().split("\n").collect();
    for line in lines {
        //let parsed:Vec<Vec<i32>> = 
        let parsed:Vec<Vec<i32>> = line.split(",")
                .map(|x| x.split("-").map(|y| y.parse::<i32>().unwrap()).collect())
                .collect();

        let entry1 = Entry{
            start: parsed[0][0],
            end: parsed[0][1],
        };

        let entry2 = Entry{
            start: parsed[1][0],
            end: parsed[1][1],
        };
        
        if is_overlapping(entry1, entry2) {
            score += 1;
        }
        
    }
    println!("ANS: {}", score);
}