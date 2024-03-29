use std::{collections::HashSet, fs};

pub fn run(path: &str) -> (String, String) {
    let input = fs::read_to_string(path).unwrap();

    let chars = input.trim().chars().collect::<Vec<char>>();

    let num = 4;

    let answer1 = num
        + chars
            .windows(num)
            .position(|window| {
                let hm = window.iter().collect::<HashSet<&char>>();
                hm.len() == window.len()
            })
            .unwrap();

    let num = 14;

    let answer2 = num
        + chars
            .windows(num)
            .position(|window| {
                let hm = window.iter().collect::<HashSet<&char>>();
                hm.len() == window.len()
            })
            .unwrap();

    (answer1.to_string(), answer2.to_string())
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn part_1() {
        let result = run("./src/day6/test1");

        assert_eq!("7", result.0);

        let result = run("./src/day6/test2");

        assert_eq!("5", result.0);

        let result = run("./src/day6/test3");

        assert_eq!("6", result.0);

        let result = run("./src/day6/test4");

        assert_eq!("10", result.0);

        let result = run("./src/day6/test5");

        assert_eq!("11", result.0);
    }

    #[test]
    fn part_2() {
        let result = run("./src/day6/test1");

        assert_eq!("19", result.1);

        let result = run("./src/day6/test2");

        assert_eq!("23", result.1);

        let result = run("./src/day6/test3");

        assert_eq!("23", result.1);

        let result = run("./src/day6/test4");

        assert_eq!("29", result.1);

        let result = run("./src/day6/test5");

        assert_eq!("26", result.1);
    }
}
