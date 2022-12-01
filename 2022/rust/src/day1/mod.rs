use std::{fs, io::BufRead};

pub fn run() {
    let input = fs::read("./src/day1/input").expect("Should have a file called `input`");
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
    println!("Most calories: {:?}", elves.last().unwrap());

    let last_three = elves.iter().rev().take(3).sum::<i32>();
    println!("Total of top three: {:?}", last_three);
}

pub fn run2() {
    let input = fs::read_to_string("./src/day1/input").unwrap();

    let mut calories = input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|item| item.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect::<Vec<i32>>();

    calories.sort_by(|a, b| b.cmp(a));
    println!(
        "{:?} : {:?}",
        calories.first().unwrap(),
        calories.iter().take(3).sum::<i32>()
    );
}
