use std::{fs, str::FromStr};

#[derive(Debug, Clone, Copy)]
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

fn is_overlapping(sprite: i32, cycle: i32) -> bool {
    cycle == sprite || cycle == sprite - 1 || cycle == sprite + 1
}

fn render(pixels: &Vec<bool>) {
    pixels.chunks(40).for_each(|line| {
        for pixel in line {
            print!("{}", if *pixel { "#" } else { " " });
        }
        print!("\n");
    })
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
    for o in ops.clone() {
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

    let mut pixels = vec![false; 240];

    let mut strength: i32 = 1;
    let mut cycle = 0;
    for o in ops {
        match o {
            Ops::Noop => {
                if is_overlapping(strength, cycle % 40) {
                    pixels[cycle as usize] = true;
                }
                cycle += 1;
            }
            Ops::Addx(x) => {
                if is_overlapping(strength, cycle % 40) {
                    pixels[cycle as usize] = true;
                }
                cycle += 1;
                if is_overlapping(strength, cycle % 40) {
                    pixels[cycle as usize] = true;
                }
                cycle += 1;
                strength += x
            }
        }
    }

    render(&pixels);

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
    fn part_2() {
        let result = run("./src/day10/test");

        assert_eq!("0", result.1);
    }
}
