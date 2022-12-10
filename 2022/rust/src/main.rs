mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day9;

fn main() {
    let arg = std::env::args().last().expect("Y U NO ARGS?!");

    let result = match arg.as_str() {
        "day1" => day1::run("./src/day1/input"),
        "day1-2" => day1::run2("./src/day1/input"),

        "day2" => day2::run("./src/day2/input"),
        "day3" => day3::run("./src/day3/input"),
        "day4" => day4::run("./src/day4/input"),
        "day5" => day5::run("./src/day5/input"),
        "day6" => day6::run("./src/day6/input"),
        "day9" => day9::run("./src/day9/input"),

        _ => panic!("Try again (:"),
    };

    println!("Result: {} - {}", result.0, result.1);
}
