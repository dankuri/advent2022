use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("src/inputs/3.txt").unwrap();
    let mut alphabet: Vec<char> = ('a'..='z').into_iter().collect();
    let mut alphabet_uppercase: Vec<char> = ('A'..='Z').into_iter().collect();
    alphabet.append(&mut alphabet_uppercase);

    // * part 1:
    let mut sum = 0;
    // dis my stinky nooby way
    for line in input.lines() {
        let (left, right) = line.split_at(line.len() / 2);
        for char in left.chars() {
            if right.contains(char) {
                sum += alphabet.iter().position(|&x| x == char).unwrap() + 1;
                break;
            }
        }
    }
    println!("{sum}");
    // dis is theprimeagen version
    const START_LOWER: u8 = b'a' - 1;
    const START_UPPER: u8 = b'A' - 1;

    let result: u32 = input
        .lines()
        .flat_map(|line| {
            let half = line.len() / 2;
            let (l, r) = line.split_at(half);
            let l = l.chars().collect::<HashSet<char>>();
            return r
                .chars()
                .filter(|c| l.contains(c))
                .collect::<HashSet<_>>()
                .into_iter()
                .map(|c| {
                    let value = if c.is_ascii_lowercase() {
                        c as u8 - START_LOWER
                    } else {
                        c as u8 - START_UPPER + 26
                    };

                    return value;
                })
                .map(|c| c as u32);
        })
        .sum::<u32>();

    println!("{result}")

    // * part 2
    // sum = 0;
}
