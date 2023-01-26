#[derive(Debug)]
struct Stack {
    crates: Vec<char>,
}

fn main() {
    let input = std::fs::read_to_string("src/inputs/5.txt").unwrap();
    let (cargo, moves) = input.split_once("\n\n").unwrap();
    let reversed_cargo = cargo.lines().rev().collect::<Vec<&str>>();
    let mut stacks: Vec<Stack> = Vec::new();

    reversed_cargo[0].chars().for_each(|char| {
        if char != ' ' {
            stacks.push(Stack { crates: Vec::new() })
        }
    });

    for line in reversed_cargo {
        let mut stack_index = 0;
        let mut index = 0;
        while index < line.len() {
            if line[index..].starts_with('[') {
                stacks[stack_index]
                    .crates
                    .push(line.chars().nth(index + 1).unwrap());
            }

            index += 4;
            stack_index += 1;
        }
    }

    moves.lines().for_each(|line| {
        let words = line.split(' ').collect::<Vec<&str>>();
        let amount = words[1].parse::<usize>().unwrap();
        let from = words[3].parse::<usize>().unwrap() - 1;
        let to = words[5].parse::<usize>().unwrap() - 1;
        // NOTE: items vec is for part 2
        let mut items: Vec<char> = Vec::new();
        for _ in 0..amount {
            items.push(stacks[from].crates.pop().unwrap());
            // let item = stacks[from].crates.pop().unwrap(); // NOTE: this is for part 1
            // stacks[to].crates.push(item);
        }
        items.reverse();
        stacks[to].crates.append(&mut items);
    });

    stacks.iter().for_each(|stack| {
        print!("{:?}", stack.crates.last().unwrap());
    });
}
