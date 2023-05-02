use std::cmp::{min, max};
use crate::utils::read_file;

fn get_bounds(line: &str) -> (i32, i32, i32, i32) {
    let assignments = line.split(",").collect::<Vec<&str>>();
    let sections_1 = assignments[0].split('-').collect::<Vec<&str>>();
    let sections_2 = assignments[1].split('-').collect::<Vec<&str>>();

    let a: i32 = sections_1[0].parse().unwrap();
    let b: i32 = sections_1[1].parse().unwrap();
    let x: i32 = sections_2[0].parse().unwrap();
    let y: i32 = sections_2[1].parse().unwrap();

    (a, b, x, y)
}

pub fn part_1() {
    let mut count = 0;
    for line in read_file("./src/input/day_4").iter() {
        let (a, b, x, y) = get_bounds(line);

        if a <= x && b >= y || x <= a && y >= b {
            count += 1;
        }
    }
    println!("{}", count);
}

pub fn part_2() {
    let mut count = 0;
    for line in read_file("./src/input/day_4").iter() {
        let (a, b, x, y) = get_bounds(line);

        if max(a, x) <= min(b, y) {
            count += 1;
        }
    }
    println!("{}", count);
}

