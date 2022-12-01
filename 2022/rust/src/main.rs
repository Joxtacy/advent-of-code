mod day1;

fn main() {
    let arg = std::env::args().last().expect("Y U NO ARGS?!");

    match arg.as_str() {
        "day1" => day1::run(),
        _ => eprintln!("Try again (:"),
    }
}
