use std::fs;

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
