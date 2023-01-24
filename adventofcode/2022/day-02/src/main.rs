use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Copy)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    pub fn points(&self) -> i32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    pub fn play(me: &Self, oponent: &Self) -> i32 {
        match (me, oponent) {
            // Win
            (Self::Rock, Self::Scissors) => 6 + me.points(),
            (Self::Paper, Self::Rock) => 6 + me.points(),
            (Self::Scissors, Self::Paper) => 6 + me.points(),

            // Lose
            (Self::Paper, Self::Scissors) => me.points(),
            (Self::Scissors, Self::Rock) => me.points(),
            (Self::Rock, Self::Paper) => me.points(),

            // Tie
            _ => 3 + me.points(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParsePointError;

impl FromStr for RPS {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            // Oponent
            "A" => Ok(Self::Rock),
            "B" => Ok(Self::Paper),
            "C" => Ok(Self::Scissors),

            // Self
            "X" => Ok(Self::Rock),
            "Y" => Ok(Self::Paper),
            "Z" => Ok(Self::Scissors),

            _ => Err(ParsePointError),
        }
    }
}

#[derive(Debug, PartialEq)]
enum ExpectedResult {
    Lose,
    Tie,
    Win,
}

impl ExpectedResult {
    fn get_hand(&self, oponent: RPS) -> RPS {
        match (self, oponent) {
            (Self::Win, RPS::Rock) => RPS::Paper,
            (Self::Win, RPS::Paper) => RPS::Scissors,
            (Self::Win, RPS::Scissors) => RPS::Rock,

            (Self::Lose, RPS::Rock) => RPS::Scissors,
            (Self::Lose, RPS::Paper) => RPS::Rock,
            (Self::Lose, RPS::Scissors) => RPS::Paper,

            (Self::Tie, a) => a,
        }
    }
}

impl FromStr for ExpectedResult {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Tie),
            "Z" => Ok(Self::Win),

            _ => Err(ParsePointError),
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let result1: i32 = input
        .lines()
        .map(
            |line| match line.split_whitespace().collect::<Vec<&str>>()[..] {
                [oponent, me] => {
                    let oponent = oponent.parse::<RPS>().unwrap();
                    let me = me.parse::<RPS>().unwrap();

                    RPS::play(&me, &oponent)
                }
                _ => panic!("Unexpected line {:?}", line),
            },
        )
        .sum();

    let result2: i32 = input
        .lines()
        .map(
            |line| match line.split_whitespace().collect::<Vec<&str>>()[..] {
                [oponent, expected] => {
                    let oponent = oponent.parse::<RPS>().unwrap();
                    let expected = expected.parse::<ExpectedResult>().unwrap();
                    let me = expected.get_hand(oponent);

                    RPS::play(&me, &oponent)
                }
                _ => panic!("Unexpected line {:?}", line),
            },
        )
        .sum();

    println!("Part 1 {}", result1);
    println!("Part 2 {}", result2);
}
