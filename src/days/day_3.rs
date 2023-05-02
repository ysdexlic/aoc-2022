use crate::utils::read_file;

fn get_byte_value(byte: char) -> u32 {
    let mut byte_num = byte as u32;
    if byte.is_ascii_lowercase() {
        byte_num -= 96;
    } else {
        byte_num -= 38;
    };
    byte_num
}

pub fn part_1() {
    let mut sum = 0;

    for line in read_file("./src/input/day_3").iter() {
        let strs = line.split_at(line.len() / 2);
        for char in strs.0.chars() {
            if strs.1.contains(char) {
                sum += get_byte_value(char);
                break;
            }
        }
    }

    println!("{}", sum)
}

pub fn part_2() {
    let mut sum = 0;
    let mut strs: Vec<&str> = vec![];

    for (index, line) in read_file("./src/input/day_3").iter().enumerate() {
        strs.push(line);
        if (index + 1) % 3 == 0 {
            for char in strs[0].chars() {
                if strs[1].contains(char) && strs[2].contains(char) {
                    sum += get_byte_value(char);
                    break;
                }
            }
            strs.clear();
        };
    }

    println!("{}", sum);
}
