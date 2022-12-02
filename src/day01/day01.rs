use std::fs;

pub fn part1() {
    let mut sums: Vec<u32> = vec![];
    let data: String = fs::read_to_string("./src/day01/input.txt").expect("Cannot read file");
    //println!("{}", data);

    let elves:Vec<&str> = data.split("\n\n").collect();
    let elves:Vec<Vec<&str>> = elves.iter().map(|x| x.split("\n").collect()).collect();
    
    for elf in elves {
        let mut sum :u32 = 0;
        for num in elf {
            let num: u32 = match num.parse() {
                Err(_e) => 0,
                Ok(n) => n,
            };
            sum += num;          
        }
        sums.push(sum);
    }
    //println!("{:?}", sums);
    let maxvalue = sums.iter().max();
    match maxvalue {
        Some(max) => println!("ANS: {}", max),
        None => println!("Empty vector?"),
    };
}

pub fn part2() {
    let mut sums: Vec<u32> = vec![];
    let data: String = fs::read_to_string("./src/day01/input.txt").expect("Cannot read file");
    //println!("{}", data);

    let elves:Vec<&str> = data.split("\n\n").collect();
    let elves:Vec<Vec<&str>> = elves.iter().map(|x| x.split("\n").collect()).collect();
    
    for elf in elves {
        let mut sum :u32 = 0;
        for num in elf {
            let num: u32 = match num.parse() {
                Err(_e) => 0,
                Ok(n) => n,
            };
            sum += num;          
        }
        sums.push(sum);
    }
    
    sums.sort();
    sums.reverse();
    let sums: u32 = sums[0] + sums[1] + sums[2];
    println!("ANS: {}", sums);
}