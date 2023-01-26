use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("src/inputs/6.txt").unwrap();
    let mut index = 0;
    let mut count = 0;
    let mut seen_chars: HashMap<char, usize> = HashMap::new();

    while index < input.len() && count < 4 {
        let char = input.chars().nth(index).unwrap();
        if seen_chars.contains_key(&char) {
            count = 0;
            index = *seen_chars.get(&char).unwrap();
            seen_chars.clear();
        } else {
            count += 1;
            seen_chars.insert(char, index);
        }
        index += 1;
    }
    println!("Part 1: {index}");

    index = 0;
    count = 0;
    seen_chars.clear();

    while index < input.len() && count < 14 {
        let char = input.chars().nth(index).unwrap();
        if seen_chars.contains_key(&char) {
            count = 0;
            index = *seen_chars.get(&char).unwrap();
            seen_chars.clear();
        } else {
            count += 1;
            seen_chars.insert(char, index);
        }
        index += 1;
    }
    println!("Part 2: {index}");
}
