fn main() {
    let calories = include_str!("../input.txt");

    let calories = calories
        .split("\n\n")
        .fold((0, 0, 0), |(top1, top2, top3), elf_calories| {
            let sum = elf_calories
                .lines()
                .map(|cal| cal.parse::<i32>().unwrap())
                .sum::<i32>();

            match sum {
                sum if sum > top1 => (sum, top1, top2),
                sum if sum > top2 => (top1, sum, top2),
                sum if sum > top3 => (top1, top2, sum),
                _ => (top1, top2, top3),
            }
        });

    let (top1, top2, top3) = calories;

    println!("Result 1 {:?}", top1);
    println!("Result 2 {:?}", top1 + top2 + top3);
}
