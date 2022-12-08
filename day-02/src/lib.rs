pub fn process_part1(input: &str) -> str {
    *"Part1"
}

pub fn process_part2(input: &str) -> str {
    *"Part2"
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

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