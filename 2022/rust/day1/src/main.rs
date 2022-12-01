use std::{fs, io::BufRead};

fn main() {
    let input = fs::read("./input.txt").expect("Should have a file called input.txt");
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
    println!("Elf carrying highest calories: {:?}", elves.last().unwrap());

    let last_three = elves.iter().rev().take(3).sum::<i32>();

    println!("Total of top three: {:?}", last_three);
}
