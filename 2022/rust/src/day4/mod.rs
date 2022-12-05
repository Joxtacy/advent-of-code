use std::{fs, ops::RangeInclusive};

pub fn run(path: &str) -> (String, String) {
    let input = fs::read_to_string(path).unwrap();

    let data = input.lines().map(|line| {
        let pair = line.split(',').collect::<Vec<&str>>();
        pair.iter()
            .map(|p| p.split('-').collect::<Vec<&str>>())
            .map(|p| p[0].parse::<i32>().unwrap()..=p[1].parse::<i32>().unwrap())
            .collect::<Vec<RangeInclusive<i32>>>()
    });

    let sum1 = data
        .clone()
        .filter_map(|pair| {
            let r1 = &pair[0];
            let r2 = &pair[1];
            if r2.contains(r1.start()) && r2.contains(r1.end()) {
                Some(1)
            } else if r1.contains(r2.start()) && r1.contains(r2.end()) {
                Some(1)
            } else {
                None
            }
        })
        .collect::<Vec<i32>>()
        .len();

    let sum2 = data
        .filter_map(|pair| {
            let r1 = &pair[0];
            let r2 = &pair[1];
            if r2.contains(r1.start()) || r2.contains(r1.end()) {
                Some(1)
            } else if r1.contains(r2.start()) || r1.contains(r2.end()) {
                Some(1)
            } else {
                None
            }
        })
        .collect::<Vec<i32>>()
        .len();

    (sum1.to_string(), sum2.to_string())
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn part_1() {
        let result = run("./src/day4/test");

        assert_eq!("2", result.0);
    }

    #[test]
    #[ignore]
    fn part_2() {
        let result = run("./src/day4/test");

        assert_eq!("4", result.1);
    }
}
