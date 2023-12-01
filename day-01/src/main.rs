fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    const RADIX: u32 = 10;
    input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|ch| ch.to_digit(RADIX))
                .collect::<Vec<u32>>()
        })
        .map(|line| match (line.first(), line.last()) {
            (Some(first), Some(last)) => 10 * first + last,
            _ => 0,
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    let new_input: Vec<String> = input
        .trim()
        .lines()
        .map(|line| {
            line.replace("one", "one1one")
                .replace("two", "two2two")
                .replace("three", "three3three")
                .replace("four", "4")
                .replace("five", "five5five")
                .replace("six", "6")
                .replace("seven", "7")
                .replace("eight", "eight8eight")
                .replace("nine", "nine9nine")
        })
        .collect();

    part1(&new_input.join("\n"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(part1(input), 142);
    }

    #[test]
    fn test_part2() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(part2(input), 281);
    }
}
