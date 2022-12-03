use std::{cmp::Ordering, fs, str::FromStr};

#[derive(PartialEq, Copy, Clone)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissor = 3,
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == &Move::Rock && other == &Move::Scissor {
            Some(Ordering::Greater)
        } else if self == &Move::Scissor && other == &Move::Rock {
            Some(Ordering::Less)
        } else {
            Some((*self as u8).cmp(&(*other as u8)))
        }
    }
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissor),
            ref e => Err(format!("Failed to parse: {}", e)),
        }
    }
}

#[derive(Copy, Clone)]
enum Outcome {
    Win = 6,
    Draw = 3,
    Lose = 0,
}

impl FromStr for Outcome {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::Lose),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            ref e => Err(format!("Failed to parse: {}", e)),
        }
    }
}

pub fn run(path: &str) -> (i32, i32) {
    let input = fs::read_to_string(path).unwrap();

    let sum1 = input
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|s| s.parse::<Move>().unwrap())
                .collect::<Vec<Move>>()
        })
        .map(|moves| {
            let opponent = moves[0];
            let me = moves[1];

            match &opponent.partial_cmp(&me) {
                Some(Ordering::Less) => 6 + me as i32,
                Some(Ordering::Equal) => 3 + me as i32,
                Some(Ordering::Greater) => 0 + me as i32,
                None => panic!("Oh noes. Couldn't compare"),
            }
        })
        .sum::<i32>();

    let sum2 = input
        .lines()
        .map(|line| line.split(" ").collect::<Vec<&str>>())
        .map(|moves| {
            let opponent = moves[0].parse::<Move>().unwrap();
            let outcome = moves[1].parse::<Outcome>().unwrap();

            outcome as i32
                + match outcome {
                    Outcome::Lose => {
                        let my_move = match opponent {
                            Move::Rock => Move::Scissor,
                            Move::Paper => Move::Rock,
                            Move::Scissor => Move::Paper,
                        };
                        my_move as i32
                    }
                    Outcome::Draw => opponent as i32,
                    Outcome::Win => {
                        let my_move = match opponent {
                            Move::Rock => Move::Paper,
                            Move::Paper => Move::Scissor,
                            Move::Scissor => Move::Rock,
                        };
                        my_move as i32
                    }
                }
        })
        .sum::<i32>();

    (sum1, sum2)
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn part_1() {
        let result = run("./src/day2/test");

        assert_eq!(15, result.0);
    }

    #[test]
    fn part_2() {
        let result = run("./src/day2/test");

        assert_eq!(12, result.1);
    }
}
