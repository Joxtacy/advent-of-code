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
