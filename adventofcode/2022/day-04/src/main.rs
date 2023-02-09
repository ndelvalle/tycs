use std::{error::Error, fmt};

#[derive(Debug, PartialEq, Eq)]
struct ParseError;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParseError")
    }
}

impl Error for ParseError {}

#[derive(Debug, Clone)]
struct Elf {
    from: usize,
    to: usize,
}

impl std::str::FromStr for Elf {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split("-").collect::<Vec<_>>()[..] {
            [from, to] => Ok(Self {
                from: from.parse()?,
                to: to.parse()?,
            }),
            _ => Err(Box::new(ParseError)),
        }
    }
}

#[derive(Debug, Clone)]
struct ElfPair {
    elf_a: Elf,
    elf_b: Elf,
}

impl ElfPair {
    fn is_fully_contained(&self) -> bool {
        let ElfPair { elf_a, elf_b } = self;
        // ...aaaaaaaaaa.....
        // .......bbbbb......
        let a_contains_b = elf_a.from <= elf_b.from && elf_a.to >= elf_b.to;

        // .......aaaaaa.....
        // .....bbbbbbbbbb...
        let b_contains_a = elf_a.from >= elf_b.from && elf_a.to <= elf_b.to;

        return a_contains_b || b_contains_a;
    }

    fn is_overlap(&self) -> bool {
        let ElfPair { elf_a, elf_b } = self;
        // .........aaaaaaa..
        // .......bbbbb......
        let a_from = elf_a.from >= elf_b.from && elf_a.from <= elf_b.to;

        // ...aaaaaaa........
        // .......bbbbb......
        let a_to = elf_a.to >= elf_b.from && elf_a.from <= elf_b.to;

        return a_from || a_to;
    }
}

impl std::str::FromStr for ElfPair {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split(",").collect::<Vec<_>>()[..] {
            [elf_a, elf_b] => Ok(ElfPair {
                elf_a: elf_a.parse()?,
                elf_b: elf_b.parse()?,
            }),
            _ => Err(Box::new(ParseError)),
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let input = input.trim();

    let pairs = input.lines().map(|line| line.parse::<ElfPair>().unwrap());

    let result_1 = pairs
        .clone()
        .filter(|pair| pair.is_fully_contained())
        .count();

    let result_2 = pairs.clone().filter(|pair| pair.is_overlap()).count();

    dbg!(result_1);
    dbg!(result_2);
}
