use std::{fs, io::BufRead};

/// BUG: The last elf does not get added to the array.
pub fn run(path: &str) -> (String, String) {
    let input = fs::read(path).expect("Should have a file called `input`");
    let lines = input.lines();

    let mut elves = vec![];
    let mut sum = 0;
    for line in lines {
        if let Ok(calories) = line {
            if calories.is_empty() {
                elves.push(sum);
                sum = 0;
            } else {
                sum += calories.parse::<i32>().unwrap();
            }
        }
    }

    elves.sort();
    let last = elves.last().unwrap();

    let last_three = elves.iter().rev().take(3).sum::<i32>();

    (last.to_string(), last_three.to_string())
}

pub fn run2(path: &str) -> (String, String) {
    let input = fs::read_to_string(path).unwrap();

    let mut calories = input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|item| item.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect::<Vec<i32>>();

    calories.sort_by(|a, b| b.cmp(a));

    let last = calories.first().unwrap();
    let last_three = calories.iter().take(3).sum::<i32>();

    (last.to_string(), last_three.to_string())
}

#[cfg(test)]
mod tests {
    use super::{run, run2};

    #[test]
    fn part_1_1() {
        let path = "./src/day1/test";

        let result = run(path);

        assert_eq!(result.0, "24000");
    }

    #[test]
    fn part_2_1() {
        let path = "./src/day1/test";

        let result = run(path);

        assert_eq!(result.1, "45000");
    }

    #[test]
    fn part_1_2() {
        let path = "./src/day1/test";

        let result = run2(path);

        assert_eq!(result.0, "24000");
    }

    #[test]
    fn part_2_2() {
        let path = "./src/day1/test";

        let result = run2(path);

        assert_eq!(result.1, "45000");
    }
}
