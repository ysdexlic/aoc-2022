use std::collections::BTreeMap;
use std::{fs::File, io::{BufReader, BufRead}};

fn rank_elves() -> BTreeMap<i32, i32> {
    let file = File::open("./src/input/day_1").unwrap();
    let reader = BufReader::new(file);

    let mut elves = BTreeMap::new();

    let mut elf = 1;
    let mut calories = 0;

    for line in reader.lines() {
        let l = line.unwrap();
        if l.is_empty() {
            elves.insert(calories, elf);
            elf += 1;
            calories = 0;
        } else {
            let num = l.parse::<i32>().unwrap();
            calories += num;
        }
    }

    elves
}

pub fn part_1() {
    let mut elves = rank_elves();
    println!("{}", elves.last_entry().unwrap().key());
}

pub fn part_2() {
    let mut elves = rank_elves();
    let top_three = vec![elves.pop_last().unwrap().0, elves.pop_last().unwrap().0, elves.pop_last().unwrap().0];
    let sum: i32 = top_three.iter().sum();
    println!("{}", sum);
}
