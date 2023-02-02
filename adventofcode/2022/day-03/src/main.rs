use std::collections::{HashMap, HashSet};

struct Rucksack {
    compartment_a: String,
    compartment_b: String,
}

impl Rucksack {
    fn get_duplicate(&self) -> Option<char> {
        let compartment_a = self.compartment_a.chars().collect::<HashSet<_>>();
        self.compartment_b
            .chars()
            .into_iter()
            .find(|c| compartment_a.contains(c))
    }
}

impl std::str::FromStr for Rucksack {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let len = s.len();
        let (compartment_a, compartment_b) = s.split_at(len / 2);

        Ok(Rucksack {
            compartment_a: compartment_a.to_string(),
            compartment_b: compartment_b.to_string(),
        })
    }
}

struct ElfGroup {
    elf_a: String,
    elf_b: String,
    elf_c: String,
}

impl ElfGroup {
    fn new(elf_a: &str, elf_b: &str, elf_c: &str) -> Self {
        ElfGroup {
            elf_a: elf_a.to_string(),
            elf_b: elf_b.to_string(),
            elf_c: elf_c.to_string(),
        }
    }

    fn get_group_badge(&self) -> char {
        let bag_a = self.elf_a.chars().collect::<HashSet<_>>();
        let bag_b = self.elf_b.chars().collect::<HashSet<_>>();

        self.elf_c
            .chars()
            .into_iter()
            .find(|c| bag_a.contains(c) && bag_b.contains(c))
            .unwrap()
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let input = input.trim();

    let chars_priority = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(index, c)| (c, index + 1));

    let chars_priority: HashMap<char, usize> = HashMap::from_iter(chars_priority);

    let result_1 = input
        .lines()
        .map(|line| line.parse::<Rucksack>().unwrap())
        .map(|rucksack| rucksack.get_duplicate().unwrap())
        .map(|c| chars_priority.get(&c).unwrap())
        .sum::<usize>();

    let result_2 = input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|chunk| {
            match chunk {
                [elf_a, elf_b, elf_c] => ElfGroup::new(elf_a, elf_b, elf_c),
                _ => panic!("Chunk is not 3 elements"),
            }
        })
        .map(|elf_group| elf_group.get_group_badge())
        .map(|c| chars_priority.get(&c).unwrap())
        .sum::<usize>();

    dbg!(result_1);
    dbg!(result_2);
}
