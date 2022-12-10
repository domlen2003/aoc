pub fn process_part1(input: &str) -> String {
    "Part1"
}

pub fn process_part2(input: &str) -> String {
    "Part2"
}


#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "input";
    #[test]
    fn test_part1() {
        assert_eq!(
            process_part1(INPUT),
            "Part1"
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            process_part2(INPUT),
            "Part2"
        );
    }
}