pub fn process_part1(input: &str) -> u32 {
    input.lines().map(|game| {
        calc_part1(get_game_nums(game))
    }).collect::<Vec<u32>>().iter().sum()
}

pub fn process_part2(input: &str) -> u32 {
    input.lines().map(|game| {
        calc_part2(get_game_nums(game))
    }).collect::<Vec<u32>>().iter().sum()
}

/*
 Rock = 0 ,
 Paper = 1,
 Scissors = 2

Wins:
 a - b
 0 - 2
 2 - 1
 1 - 0

 win if b - a % 3 == 2 (signed module)
 unsigned version -> b +3 - a % 3 == 2
*/
fn calc_part1(game: (u32, u32)) -> u32 {
    (game.1 + 1) + if (game.0 + 3 - game.1) % 3 == 2 { 6 } else if game.0 == game.1 { 3 } else { 0 }
}

fn calc_part2(game: (u32, u32)) -> u32 {
    //game.1 is the result
    match game.1 {
        0 => ((game.0 + 2) % 3) + 1 + 0,//(opponent+2)%3  is a the lose sign    +1 for the sign value +0 for the lose
        1 => game.0 + 1 + 3,//draw sign is the same sign as the opponent         +1 for the sign value  +3 for the draw
        2 => ((game.0 + 1) % 3) + 1 + 6,//(opponent+2)%3  is a the win sign      +1 for the sign value +6 for the win
        _ => unreachable!(),
    }
}

fn get_game_nums(game: &str) -> (u32, u32) {
    let (opponent, own) = game.split_once(" ").unwrap();
    (opponent.as_bytes()[0] as u32 - 65, own.as_bytes()[0] as u32 - 88)
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "A Y\nB X\nC Z";
    #[test]
    fn test_part1() {
        assert_eq!(
            process_part1(INPUT),
            15
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            process_part2(INPUT),
            12
        );
    }
}