use std::collections::HashMap;

#[derive(Debug)]
struct CrateId {
    id: char,
}

impl CrateId {
    fn from_chars(chars: Vec<char>) -> Option<Self> {
        match chars.get(1) {
            None => None,
            Some(id) if id.is_whitespace() => None,
            Some(id) => Some(CrateId { id: *id }),
        }
    }
}

#[derive(Debug)]
struct Operation {
    amount: usize,
    from: usize,
    to: usize,
}

impl From<&str> for Operation {
    fn from(s: &str) -> Self {
        let parts = s.split_whitespace().collect::<Vec<_>>();
        let amount = parts.get(1).unwrap().parse().unwrap();
        let from = parts.get(3).unwrap().parse().unwrap();
        let to = parts.get(5).unwrap().parse().unwrap();

        Operation { amount, from, to }
    }
}

type Crates = HashMap<usize, Vec<CrateId>>;

fn operate(crates: &mut Crates, operation: Operation) {
    for _ in 0..operation.amount {
        let crate_id = crates.get_mut(&operation.from).unwrap().pop().unwrap();
        crates.get_mut(&operation.to).unwrap().push(crate_id);
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let (stacks, instructions) = input.split_once("\n\n").unwrap();

    let mut crates = Crates::new();
    stacks
        .lines()
        .rev()
        .skip(1)
        .map(|line| String::from(line))
        .for_each(|line| {
            line.chars()
                .collect::<Vec<_>>()
                .chunks(4)
                .map(|chunk| CrateId::from_chars(chunk.to_vec()))
                .enumerate()
                .for_each(|(index, crated_id)| {
                    if let Some(crate_id) = crated_id {
                        crates.entry(index + 1).or_insert(Vec::new()).push(crate_id);
                    }
                });
        });

    instructions.lines().for_each(|line| {
        let operation = Operation::from(line);
        operate(&mut crates, operation);
    });

    let mut result = crates
        .into_iter()
        .map(|(stack_position, mut stack)| (stack_position, stack.pop().unwrap()))
        .collect::<Vec<_>>();

    result.sort_by_key(|(stack_position, _)| *stack_position);

    let result = result
        .into_iter()
        .map(|(_, crate_id)| crate_id.id)
        .collect::<String>();

    println!("Result part 1: {result}");
}
