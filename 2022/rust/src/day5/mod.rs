use std::{fs, str::FromStr};

#[derive(Debug)]
struct Move {
    from: i32,
    to: i32,
    count: i32,
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let i = s
            .split_ascii_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .collect::<Vec<i32>>();

        Ok(Move {
            count: i[0],
            from: i[1],
            to: i[2],
        })
    }
}

#[derive(Debug)]
struct Stack<T> {
    stack: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { stack: Vec::new() }
    }

    fn push(&mut self, item: T) {
        self.stack.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.stack.pop()
    }
}

pub fn run(path: &str) -> (String, String) {
    let input = fs::read_to_string(path).unwrap();

    let input = input.split("\n\n").collect::<Vec<&str>>();

    let boxes = input[0];

    let mut boxes = boxes.lines().collect::<Vec<&str>>();
    boxes.reverse();

    let l = (boxes[0].len() + 1) / 4;

    let mut stacks: Vec<Stack<char>> = vec![];
    let mut vecs: Vec<Vec<char>> = vec![];
    for _ in 0..l {
        stacks.push(Stack::new());
        vecs.push(vec![]);
    }

    for row in boxes {
        let length = (row.len() + 1) / l;

        row.chars().enumerate().for_each(|(i, c)| match c {
            'a'..='z' | 'A'..='Z' => {
                let index = i / length;
                stacks[index].push(c);
                vecs[index].push(c);
            }
            _ => {}
        })
    }

    let moves = input[1];
    let moves = moves
        .lines()
        .filter_map(|m| m.parse().ok())
        .collect::<Vec<Move>>();

    for mov in moves {
        let Move { to, from, count } = mov;

        let mut helper: Vec<char> = vec![];
        for _ in 0..count {
            let b = stacks[(from - 1) as usize].pop();
            stacks[(to - 1) as usize].push(b.unwrap());

            let a = vecs[(from - 1) as usize].pop();
            helper.push(a.unwrap());
        }

        for _ in 0..count {
            vecs[(to - 1) as usize].push(helper.pop().unwrap());
        }
    }

    let mut s1 = vec![];
    for mut stack in stacks {
        s1.push(stack.pop().unwrap().to_string());
    }
    let mut s2 = vec![];
    for mut y in vecs {
        s2.push(y.pop().unwrap().to_string());
    }

    let sum1 = s1.join("");
    let sum2 = s2.join("");

    (sum1, sum2)
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn part_1() {
        let result = run("./src/day5/test");

        assert_eq!("CMZ", result.0);
    }

    #[test]
    fn part_2() {
        let result = run("./src/day5/test");

        assert_eq!("MCD", result.1);
    }
}
