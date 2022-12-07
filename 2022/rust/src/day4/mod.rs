use nom::bytes::complete::tag;
use nom::character::complete;
use nom::multi::separated_list0;
use nom::sequence::separated_pair;
use nom::IResult;
use std::{fs, ops::RangeInclusive};

/// Parses out a section as a `RangeInclusive`.
///
/// Example: "2-4" -> 2..=4
fn sections(input: &str) -> IResult<&str, RangeInclusive<i32>> {
    let (input, (start, end)) = separated_pair(complete::i32, tag("-"), complete::i32)(input)?;

    Ok((input, start..=end))
}

/// Parses out a pair of sections separated by a comma as a tuple of `RangeInclusive`.
///
/// Example: "2-4,5-6" -> (2..=4, 5..=6)
fn pair(input: &str) -> IResult<&str, (RangeInclusive<i32>, RangeInclusive<i32>)> {
    let (input, pair) = separated_pair(sections, tag(","), sections)(input)?;

    Ok((input, pair))
}

/// Parses out a list of pairs separated by newlines as a `Vec` of tuples of `RangeInclusive`.
///
/// Example: "2-4,5-6\n2-7,3-6" => vec![(2..=4, 5..=5), (2..=7, 3..=6)]
fn pairs(input: &str) -> IResult<&str, Vec<(RangeInclusive<i32>, RangeInclusive<i32>)>> {
    let (input, pairs) = separated_list0(tag("\n"), pair)(input)?;

    Ok((input, pairs))
}

pub fn run(path: &str) -> (String, String) {
    let input = fs::read_to_string(path).unwrap();

    let (_, pairs) = pairs(input.as_str()).unwrap();

    let sum1 = pairs
        .iter()
        .filter_map(|(section1, section2)| {
            if section1.contains(section2.start()) && section1.contains(section2.end()) {
                Some(())
            } else if section2.contains(section1.start()) && section2.contains(section1.end()) {
                Some(())
            } else {
                None
            }
        })
        .count();

    let sum2 = pairs
        .iter()
        .filter_map(|(section1, section2)| {
            if section1.contains(section2.start()) || section1.contains(section2.end()) {
                Some(())
            } else if section2.contains(section1.start()) || section2.contains(section1.end()) {
                Some(())
            } else {
                None
            }
        })
        .count();

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
