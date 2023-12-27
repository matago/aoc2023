advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let val: u32 = input
        .lines()
        .map(|line| {
            let mut digits = line.chars().filter_map(|c| c.to_digit(10));
            let first_digit = digits.next().unwrap();
            let last_digit = digits.next_back().unwrap_or(first_digit);
            10 * first_digit + last_digit
        })
        .sum();
    Some(val)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
