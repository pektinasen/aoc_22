use std::env;
use std::fs;

fn part_one(lines: &Vec<&str>) {
    let data = lines.into_iter()
        .map(|elf| elf.parse::<u32>());
    
    let mut elves: Vec<Vec<u32>> = Vec::new();
    let mut elf: Vec<u32> = Vec::new();
    for d in data {
        if let(Ok(dd)) = d {
            elf.push(dd);
        } else {
            elves.push(elf);
            elf = Vec::new();
        }
    }

    let mut elves: Vec<u32> = elves
        .into_iter()
        .map(|elf| elf.into_iter().reduce(|a, b| a + b).unwrap())
        .collect();
    elves.sort();
    // +1 because enumerate starts at 0 which doesn't semantically match the nth elf
    //println!("TEXT \n\n{:?}", data.last());
    println!("{:?}", elves.last());
}

fn part_two(lines: &Vec<&str>) -> Option<u32> {
    let data = lines.into_iter()
        .map(|elf| elf.parse::<u32>());
    
    let mut elves: Vec<Vec<u32>> = Vec::new();
    let mut elf: Vec<u32> = Vec::new();
    for d in data {
        if let(Ok(dd)) = d {
            elf.push(dd);
        } else {
            elves.push(elf);
            elf = Vec::new();
        }
    }
    let mut elves: Vec<u32> = elves
        .into_iter()
        .map(|elf| elf.into_iter().reduce(|a, b| a + b).unwrap())
        .collect();
    elves.sort();
    
    elves.reverse();
    elves.as_slice()[elves.len()-3..].to_vec().into_iter().reduce(|a,b| a + b).map(|result| {
        println!("{:?}", elves[0] + elves[1] + elves[2]);
        result
    })
}

