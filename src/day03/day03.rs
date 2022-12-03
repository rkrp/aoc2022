use std::fs;
use std::collections::HashSet;

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_get_intersection() {
        let str1 = "vJrwpWtwJgWrhcsFMMfFFhFp".to_string();
        let str2 = "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string();
        let str3 = "PmmdzqPrVvPwwTWBwg".to_string();
        
        let str1:HashSet<char> = HashSet::from_iter(str1.chars());
        let str2:HashSet<char> = HashSet::from_iter(str2.chars());
        let str3:HashSet<char> = HashSet::from_iter(str3.chars());

        let group:Vec<HashSet<char>> = Vec::from([str1, str2, str3]);

        let result = get_intersection_three(group);
        assert_eq!(result, vec!['r']);
    }
}


fn get_score(a:char) -> u32{
    let ascii  = a as u32;

    if ascii >= 97 && ascii <= 122 {
        ascii - 96
    } else if ascii >= 65 && ascii <= 90 {
        ascii - 38
    } else {
        panic!("Also unreachable!");
    }
}

fn get_intersection(comp1: &str, comp2: &str) -> char {
    let comp1:Vec<char> = comp1.chars().collect();
    let comp2:Vec<char> = comp2.chars().collect();

    let comp1:HashSet<char> = HashSet::from_iter(comp1);
    let comp2:HashSet<char> = HashSet::from_iter(comp2);

    for i in comp1.intersection(&comp2) {
        return *i;
    }
    panic!("Unreachable!");
}

pub fn part1() {
    let data: String = fs::read_to_string("./src/day03/input.txt").expect("Cannot read file");
    let mut score = 0;
    let lines:Vec<&str> = data.trim().split("\n").collect();
    for line in lines {
        let line = line.trim();
        let len = line.len() / 2;
        
        let result = get_intersection(&line[..len], &line[len..]);
        score += get_score(result);
    }
    println!("ANS: {}", score);
}

fn get_intersection_three(group:Vec<HashSet<char>>) -> Vec<char>{
    if group.len() != 3 {
        panic!("Invalid length group received");
    }
    
    let hash1 = group.get(0).unwrap();
    let hash2 = group.get(1).unwrap();
    let hash3 = group.get(2).unwrap();

    //let foo= Vec::from_iter(hash1.intersection(hash2));
    let result:HashSet<char> = HashSet::from_iter(hash1.intersection(hash2).into_iter().cloned());
    //let result = HashSet::from_iter(result.into_iter().cloned());
    let result = Vec::from_iter(result.intersection(hash3).into_iter().cloned());

    result
    
    //let result = Vec::from_iter::<char>(hash3.intersection(&temp).collect());
}

pub fn part2() {
    let data: String = fs::read_to_string("./src/day03/input.txt").expect("Cannot read file");

    let lines:Vec<&str> = data.trim().split("\n").collect();
    let mut hashy:HashSet<char>;
    let mut group: Vec<HashSet<char>> = vec![];

    let mut group_counter = 0;
    let mut score = 0;
    for line in lines {
        hashy = HashSet::from_iter(line.chars());
        if group_counter < 3 {
            group.push(hashy);
        }
        group_counter += 1;
        if group_counter == 3 {
            group_counter = 0;
            let result = get_intersection_three(group);
            let result = result.get(0).unwrap();
            score += get_score(*result);
            group = vec![];
        }
        
    }
    println!("ANS: {}", score);
  
}