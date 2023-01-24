use std::str::FromStr;

#[derive(Debug, PartialEq)]
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

    pub fn play(&self, oponent: &Self) -> i32 {
        match (self, oponent) {
            // Win
            (Self::Rock, Self::Scissors) => 6 + self.points(),
            (Self::Paper, Self::Rock) => 6 + self.points(),
            (Self::Scissors, Self::Paper) => 6 + self.points(),

            // Lose
            (Self::Paper, Self::Scissors) => self.points(),
            (Self::Scissors, Self::Rock) => self.points(),
            (Self::Rock, Self::Paper) => self.points(),

            // Tie
            _ => 3 + self.points(),
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

fn main() {
    let input = include_str!("../input.txt");

    let result1: i32 = input
        .lines()
        .map(
            |line| match line.split_whitespace().collect::<Vec<&str>>()[..] {
                [oponent, me] => {
                    let oponent = oponent.parse::<RPS>().unwrap();
                    let me = me.parse::<RPS>().unwrap();

                    me.play(&oponent)
                }
                _ => panic!("Unexpected line {:?}", line),
            },
        )
        .sum();

    println!("{}", result1);
}
