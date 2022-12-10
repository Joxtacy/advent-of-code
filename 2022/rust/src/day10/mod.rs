use std::{fs, str::FromStr};

#[derive(Debug)]
enum Ops {
    Noop,
    Addx(i32),
}

impl FromStr for Ops {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "noop" {
            Ok(Ops::Noop)
        } else {
            let mut split = s.split_whitespace();
            split.next();
            let signal = split.next().unwrap().parse::<i32>().unwrap();
            Ok(Ops::Addx(signal))
        }
    }
}

pub fn run(path: &str) -> (String, String) {
    let input = fs::read_to_string(path).unwrap();

    let cycles = vec![20, 60, 100, 140, 180, 220];

    let ops = input
        .lines()
        .map(|line| line.parse::<Ops>().unwrap())
        .collect::<Vec<Ops>>();

    let mut sum = 0;
    let mut strength: i32 = 1;
    let mut cycle = 1;
    for o in ops {
        if cycles.contains(&cycle) {
            sum += strength * cycle;
        }

        match o {
            Ops::Noop => {
                cycle += 1;
                continue;
            }
            Ops::Addx(x) => {
                cycle += 1;
                if cycles.contains(&cycle) {
                    sum += strength * cycle;
                }
                cycle += 1;
                strength += x
            }
        }
    }

    let answer1 = sum;
    let answer2 = 0;
    (answer1.to_string(), answer2.to_string())
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn part_1() {
        let result = run("./src/day10/test");

        assert_eq!("13140", result.0);
    }

    #[test]
    #[ignore]
    fn part_2() {
        let result = run("./src/day10/test");

        assert_eq!("36", result.1);
    }
}
