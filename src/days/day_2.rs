use std::{fs::File, io::{BufReader, BufRead}};

fn get_move_score(key: &str) -> i16 {
    match key {
        "A" | "X" => 1,
        "B" | "Y" => 2,
        "C" | "Z" => 3,
        &_ => 0,
    }
}

fn get_round_score(elf_move: &str, my_move: &str) -> i16 {
    let elf_move = get_move_score(elf_move);
    let my_move = get_move_score(my_move);

    let mut score = 0;
    if my_move - elf_move == 0 {
        // draw
        score += 3;
    }
    else if my_move - elf_move == 1 || my_move - elf_move == -2 {
        // win
        score += 6;
    }
    score + my_move
}

pub fn part_1() {
    let file = File::open("./src/input/day_2").unwrap();
    let reader = BufReader::new(file);

    let mut score = 0;
    for line in reader.lines() {
        let l = line.unwrap();
        let moves: Vec<&str> = l.split(' ').collect();

        score += get_round_score(moves[0], moves[1])
    }
    println!("{}", score);
}

pub fn part_2() {
    let file = File::open("./src/input/day_2").unwrap();
    let reader = BufReader::new(file);

    let possible_moves = ["A", "B", "C"];

    let mut score = 0;
    for line in reader.lines() {
        let l = line.unwrap();
        let moves: Vec<&str> = l.split(' ').collect();

        let elf_index = possible_moves.iter().position(|&r| r == moves[0]).unwrap();

        let mut move_to_make = "";
        match moves[1] {
            "X" => move_to_make = possible_moves[(elf_index + possible_moves.len() - 1) % 3],
            "Y" => move_to_make = moves[0],
            "Z" => move_to_make = possible_moves[(elf_index + possible_moves.len() + 1) % 3],
            _ => (),
        };

        score += get_round_score(moves[0], move_to_make)
    }
    println!("{}", score);
}
