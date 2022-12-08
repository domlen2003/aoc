pub fn process_part1(input: &str) -> u32 {
    input.split("\n").map(|game| {
        let signs: Vec<&str> = game.split(" ").collect();
        let opponent = get_sign(signs[0]);
        let own = get_sign(signs[1]);
        calc_points(&own, &calc_result(&own, &opponent))
    }).collect::<Vec<u32>>().iter().sum()
}


pub fn process_part2(input: &str) -> u32 {
    input.split("\n").map(|game| {
        let signs: Vec<&str> = game.split(" ").collect();
        let opponent = get_sign(signs[0]);
        let target = get_target(signs[1]);
        calc_points(&calc_sign(&opponent, &target), &target)
    }).collect::<Vec<u32>>().iter().sum()
}


enum Result {
    WIN,
    LOSE,
    DRAW,
}

enum Sign {
    ROCK,
    PAPER,
    SCISSORS,
}

fn calc_points(sign: &Sign, outcome: &Result) -> u32 {
    let outcome_val = match outcome {
        Result::WIN => 6,
        Result::LOSE => 0,
        Result::DRAW => 3,
    };
    let sign_val = match sign {
        Sign::ROCK => 1,
        Sign::PAPER => 2,
        Sign::SCISSORS => 3,
    };
    outcome_val + sign_val
}

fn calc_sign(opponent: &Sign, target: &Result) -> Sign {
    match target {
        Result::WIN => match opponent {
            Sign::ROCK => Sign::PAPER,
            Sign::PAPER => Sign::SCISSORS,
            Sign::SCISSORS => Sign::ROCK,
        },
        Result::LOSE => match opponent {
            Sign::ROCK => Sign::SCISSORS,
            Sign::PAPER => Sign::ROCK,
            Sign::SCISSORS => Sign::PAPER,
        },
        Result::DRAW => match opponent {
            Sign::ROCK => Sign::ROCK,
            Sign::PAPER => Sign::PAPER,
            Sign::SCISSORS => Sign::SCISSORS,
        },
    }
}

fn calc_result(own: &Sign, opponent: &Sign) -> Result {
    match own {
        Sign::ROCK => match opponent {
            Sign::ROCK => Result::DRAW,
            Sign::PAPER => Result::LOSE,
            Sign::SCISSORS => Result::WIN
        },
        Sign::PAPER => match opponent {
            Sign::ROCK => Result::WIN,
            Sign::PAPER => Result::DRAW,
            Sign::SCISSORS => Result::LOSE
        },
        Sign::SCISSORS => match opponent {
            Sign::ROCK => Result::LOSE,
            Sign::PAPER => Result::WIN,
            Sign::SCISSORS => Result::DRAW,
        },
    }
}

fn get_sign(character: &str) -> Sign {
    match character {
        "A" => Sign::ROCK,
        "B" => Sign::PAPER,
        "C" => Sign::SCISSORS,
        "X" => Sign::ROCK,
        "Y" => Sign::PAPER,
        "Z" => Sign::SCISSORS,
        &_ => panic!("Unknown sign")
    }
}

fn get_target(character: &str) -> Result {
    match character {
        "X" => Result::LOSE,
        "Y" => Result::DRAW,
        "Z" => Result::WIN,
        &_ => panic!("Unknown target")
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            process_part1("A Y\nB X\nC Z"),
            15
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            process_part2("A Y\nB X\nC Z"),
            12
        );
    }
}