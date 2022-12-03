mod day1;
mod day2;
mod day3;

fn main() {
    let arg = std::env::args().last().expect("Y U NO ARGS?!");

    let result = match arg.as_str() {
        "day1" => day1::run("./src/day1/input"),
        "day1-2" => day1::run2("./src/day1/input"),

        "day2" => day2::run("./src/day2/input"),
        "day3" => day3::run("./src/day3/input"),

        _ => panic!("Try again (:"),
    };

    println!("Result: {} - {}", result.0, result.1);
}
