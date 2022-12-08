

pub fn process_part1(input: &str) -> u128 {
    *get_calories(input).iter().max().unwrap()
}

pub fn process_part2(input: &str) -> u128 {
    let mut cals = get_calories(input);
    cals.sort_by(|a, b| a.cmp(b).reverse());
    cals[..3].iter().sum()
}

fn get_calories(input: &str) -> Vec<u128> {
    input.split("\n\n")
        .map(|elve|
            elve.split("\n")
                .map(|item| item.parse::<u128>().unwrap())
                .sum()
        ).collect()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    const INPUT: &str = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";

    #[test]
    fn calories() {
        assert_eq!(
            get_calories(INPUT),
            [6000, 4000, 11000, 24000, 10000]
        );
    }


    #[test]
    fn test_part1() {
        assert_eq!(
            process_part1(INPUT),
            24000
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            process_part2(INPUT),
            45000
        );
    }
}