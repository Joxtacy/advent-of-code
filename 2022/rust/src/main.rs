mod day1;
mod day2;

fn main() {
    let arg = std::env::args().last().expect("Y U NO ARGS?!");

    match arg.as_str() {
        "day1" => {
            day1::run("./src/day1/input");
        }
        "day1-2" => {
            day1::run2("./src/day1/input");
        }
        "day2" => {
            day2::run("./src/day2/input");
        }
        _ => eprintln!("Try again (:"),
    }
}
