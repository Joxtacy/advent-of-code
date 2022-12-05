use std::fs;

fn convert(c: &char) -> i32 {
    match c.is_lowercase() {
        true => *c as i32 - 96,
        false => *c as i32 - 38,
    }
}

pub fn run(path: &str) -> (String, String) {
    let input = fs::read_to_string(path).unwrap();

    let sum1 = input
        .lines()
        .map(|line| {
            let len = line.len();
            let i = len / 2;
            let (bp1, bp2) = line.split_at(i);

            let bp1 = bp1.chars().collect::<Vec<char>>();
            let bp2 = bp2.chars();

            for c in bp2 {
                if bp1.contains(&c) {
                    return convert(&c);
                }
            }
            0
        })
        .sum::<i32>();

    let mut sum2 = 0;

    let lines = input.lines().collect::<Vec<&str>>();
    for chunk in lines.chunks(3) {
        let bp1 = chunk[0];
        let bp2 = chunk[1];
        let bp3 = chunk[2];

        for badge in bp1.chars() {
            if bp2.contains(badge) && bp3.contains(badge) {
                sum2 += convert(&badge);
                break;
            }
        }
    }

    (sum1.to_string(), sum2.to_string())
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn part_1() {
        let result = run("./src/day3/test");

        assert_eq!("157", result.0);
    }

    #[test]
    fn part_2() {
        let result = run("./src/day3/test");

        assert_eq!("70", result.1);
    }
}
