use std::{cmp::Ordering, fs, str::FromStr};

const A: &str = "A";
const B: &str = "B";
const X: &str = "X";
const Y: &str = "Y";

pub fn run(path: &str) -> (i32, i32) {
    let lines = fs::read_to_string(path).unwrap();

    let lines = lines.lines();

    let mut sum1 = 0;

    for line in lines.to_owned() {
        let mut split = line.split(" ");
        let opponent = split.next().unwrap();
        let me = split.next().unwrap();

        let mut score = if me == X {
            1
        } else if me == Y {
            2
        } else {
            3
        };

        score += if opponent == A {
            if me == X {
                3
            } else if me == Y {
                6
            } else {
                0
            }
        } else if opponent == B {
            if me == X {
                0
            } else if me == Y {
                3
            } else {
                6
            }
        } else {
            if me == X {
                6
            } else if me == Y {
                0
            } else {
                3
            }
        };

        sum1 += score;
    }

    let mut sum2 = 0;

    for line in lines {
        let mut split = line.split(" ");
        let opponent = split.next().unwrap();
        let me = split.next().unwrap();

        let mut score = if me == X {
            0
        } else if me == Y {
            3
        } else {
            6
        };

        score += if opponent == A {
            if me == X {
                3
            } else if me == Y {
                1
            } else {
                2
            }
        } else if opponent == B {
            if me == X {
                1
            } else if me == Y {
                2
            } else {
                3
            }
        } else {
            if me == X {
                2
            } else if me == Y {
                3
            } else {
                1
            }
        };

        sum2 += score;
    }

    (sum1, sum2)
}

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

pub fn run2(path: &str) -> (i32, i32) {
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
            let outcome = moves[1];

            match outcome {
                "X" => {
                    let my_move = match opponent {
                        Move::Rock => Move::Scissor,
                        Move::Paper => Move::Rock,
                        Move::Scissor => Move::Paper,
                    };
                    0 + my_move as i32
                }
                "Y" => 3 + opponent as i32,
                "Z" => {
                    let my_move = match opponent {
                        Move::Rock => Move::Paper,
                        Move::Paper => Move::Scissor,
                        Move::Scissor => Move::Rock,
                    };
                    6 + my_move as i32
                }
                _ => panic!("Oh noes. Poopie"),
            }
        })
        .sum::<i32>();

    (sum1, sum2)
}

#[cfg(test)]
mod tests {
    use super::{run, run2};

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

    #[test]
    fn part_1_2() {
        let result = run2("./src/day2/test");

        assert_eq!(15, result.0);
    }

    #[test]
    fn part_2_2() {
        let result = run2("./src/day2/test");

        assert_eq!(12, result.1);
    }
}
