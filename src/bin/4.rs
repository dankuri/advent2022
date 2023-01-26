use std::str::FromStr;

struct Task {
    start: usize,
    end: usize,
}

impl Task {
    pub fn contains(&self, other: &Task) -> bool {
        self.start <= other.start && self.end >= other.end
    }
    pub fn overlaps(&self, other: &Task) -> bool {
        self.end >= other.start && self.start <= other.end
    }
}

impl FromStr for Task {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (l, r) = s.split_once('-').expect("please split task");
        Ok(Task {
            start: l.parse()?,
            end: r.parse()?,
        })
    }
}

struct Tasks {
    left: Task,
    right: Task,
}

impl Tasks {
    pub fn joe_ate_mama(&self) -> bool {
        self.left.contains(&self.right) || self.right.contains(&self.left)
    }
    pub fn joe_almost_ate_mama(&self) -> bool {
        self.left.overlaps(&self.right) || self.right.overlaps(&self.left)
    }
}

impl FromStr for Tasks {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left_task, right_task) = s.split_once(',').expect("please split tasks");
        Ok(Tasks {
            left: left_task.parse()?,
            right: right_task.parse()?,
        })
    }
}

fn main() {
    let input = std::fs::read_to_string("src/inputs/4.txt").unwrap();
    let count_contains = input
        .lines()
        .flat_map(|line| line.parse::<Tasks>())
        .filter(|tasks| tasks.joe_ate_mama())
        .count();
    println!("Part 1: {count_contains}");

    let count_overlaps = input
        .lines()
        .flat_map(|line| line.parse::<Tasks>())
        .filter(|tasks| tasks.joe_almost_ate_mama())
        .count();
    println!("Part 2: {count_overlaps}");
}
