use std::{collections::HashSet, fs, str::FromStr};

#[derive(Debug, Clone, Copy)]
enum Move {
    R(i32),
    L(i32),
    U(i32),
    D(i32),
}

impl FromStr for Move {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();
        let mov = split.next().unwrap();
        let dist = split.next().unwrap().parse::<i32>().unwrap();

        match mov {
            "R" => Ok(Move::R(dist)),
            "L" => Ok(Move::L(dist)),
            "U" => Ok(Move::U(dist)),
            "D" => Ok(Move::D(dist)),
            _ => Err(String::from("Nope")),
        }
    }
}

fn touching(h: (i32, i32), t: (i32, i32)) -> bool {
    let d = (((h.0 - t.0).pow(2) + (h.1 - t.1).pow(2)) as f64).sqrt();
    if d >= 1.5 {
        false
    } else {
        true
    }
}

fn calc_move(h: (i32, i32), t: (i32, i32)) -> Vec<Move> {
    let x = h.0 - t.0;
    let y = h.1 - t.1;

    let mut moves = vec![];

    if x == 1 && y == 2 {
        moves.push(Move::R(1));
        moves.push(Move::U(1));
    } else if x == 1 && y == -2 {
        moves.push(Move::R(1));
        moves.push(Move::D(1));
    } else if x == -1 && y == 2 {
        moves.push(Move::L(1));
        moves.push(Move::U(1));
    } else if x == -1 && y == -2 {
        moves.push(Move::L(1));
        moves.push(Move::D(1));
    } else if x == 2 && y == 1 {
        moves.push(Move::R(1));
        moves.push(Move::U(1));
    } else if x == 2 && y == -1 {
        moves.push(Move::R(1));
        moves.push(Move::D(1));
    } else if x == -2 && y == 1 {
        moves.push(Move::L(1));
        moves.push(Move::U(1));
    } else if x == -2 && y == -1 {
        moves.push(Move::L(1));
        moves.push(Move::D(1));
    } else if x == 2 && y == 2 {
        moves.push(Move::U(1));
        moves.push(Move::R(1));
    } else if x == 2 && y == -2 {
        moves.push(Move::D(1));
        moves.push(Move::R(1));
    } else if x == -2 && y == 2 {
        moves.push(Move::U(1));
        moves.push(Move::L(1));
    } else if x == -2 && y == -2 {
        moves.push(Move::D(1));
        moves.push(Move::L(1));
    } else if x == 0 {
        if y == 2 {
            moves.push(Move::U(1));
        } else if y == -2 {
            moves.push(Move::D(1));
        }
    } else if y == 0 {
        if x == 2 {
            moves.push(Move::R(1));
        } else if x == -2 {
            moves.push(Move::L(1));
        }
    }

    moves
}

fn make_move(a: (i32, i32), mov: Move) -> (i32, i32) {
    match mov {
        Move::R(d) => (a.0 + d, a.1),
        Move::L(d) => (a.0 - d, a.1),
        Move::U(d) => (a.0, a.1 + d),
        Move::D(d) => (a.0, a.1 - d),
    }
}

pub fn run(path: &str) -> (String, String) {
    let input = fs::read_to_string(path).unwrap();

    let moves = input
        .lines()
        .filter_map(|line| line.parse::<Move>().ok())
        .collect::<Vec<Move>>();

    let mut touched: HashSet<(i32, i32)> = HashSet::new();

    let num_tails = 2;
    let mut tails = vec![(0, 0); num_tails];

    for mov in moves.clone() {
        let d = match mov {
            Move::R(d) | Move::L(d) | Move::U(d) | Move::D(d) => d,
        };
        for _ in 1..=d {
            let h = tails[0];
            tails[0] = match mov {
                Move::R(_) => (h.0 + 1, h.1),
                Move::L(_) => (h.0 - 1, h.1),
                Move::U(_) => (h.0, h.1 + 1),
                Move::D(_) => (h.0, h.1 - 1),
            };

            for n in 1..tails.len() {
                let h = tails[n - 1];
                let mut t = tails[n];

                let t_moves = calc_move(h, t);
                for m in t_moves {
                    t = make_move(t, m);
                }
                tails[n] = t;
            }
            touched.insert(tails[tails.len() - 1]);
        }
    }
    let answer1 = touched.len();

    // part 2
    let mut touched: HashSet<(i32, i32)> = HashSet::new();

    let num_tails = 10;
    let mut tails = vec![(0, 0); num_tails];

    for mov in moves {
        let d = match mov {
            Move::R(d) | Move::L(d) | Move::U(d) | Move::D(d) => d,
        };
        for _ in 1..=d {
            let h = tails[0];
            tails[0] = match mov {
                Move::R(_) => (h.0 + 1, h.1),
                Move::L(_) => (h.0 - 1, h.1),
                Move::U(_) => (h.0, h.1 + 1),
                Move::D(_) => (h.0, h.1 - 1),
            };

            for n in 1..tails.len() {
                let h = tails[n - 1];
                let mut t = tails[n];

                let t_moves = calc_move(h, t);
                for m in t_moves {
                    t = make_move(t, m);
                }
                tails[n] = t;
            }
            touched.insert(tails[tails.len() - 1]);
        }
    }

    let answer2 = touched.len();
    (answer1.to_string(), answer2.to_string())
}

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn part_1() {
        let result = run("./src/day9/test");

        assert_eq!("13", result.0);
    }

    #[test]
    fn part_2() {
        let result = run("./src/day9/test2");

        assert_eq!("36", result.1);
    }
}
