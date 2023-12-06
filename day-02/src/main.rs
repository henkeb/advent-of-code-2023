fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[derive(Debug)]
struct Game {
    id: u32,
    red: u32,
    green: u32,
    blue: u32,
}

impl Game {
    fn new(id: u32) -> Self {
        Self {
            id,
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

fn part1(input: &str) -> u32 {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let id: u32 = parts
                .next()
                .unwrap()
                .split_whitespace()
                .last()
                .unwrap()
                .parse::<u32>()
                .unwrap();
            let mut game = Game::new(id);

            let games = parts.next().unwrap().split("; ");
            for one_game in games {
                let split_one_game = one_game.split(", ");
                for number_and_color in split_one_game {
                    let mut cube = number_and_color.split_whitespace();
                    let num = cube.next().unwrap().parse::<u32>().unwrap();
                    let color = cube.next().unwrap();
                    match color {
                        "red" if num > game.red => game.red = num,
                        "green" if num > game.green => game.green = num,
                        "blue" if num > game.blue => game.blue = num,
                        _ => (),
                    }
                }
            }
            game
        })
        .filter(|game| game.red <= 12 && game.green <= 13 && game.blue <= 14)
        .map(|game| game.id)
        .sum()
}

fn part2(input: &str) -> u32 {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let mut game = Game::new(0);

            let games = parts.nth(1).unwrap().split("; ");
            for one_game in games {
                let split_one_game = one_game.split(", ");
                for number_and_color in split_one_game {
                    let mut cube = number_and_color.split_whitespace();
                    let num = cube.next().unwrap().parse::<u32>().unwrap();
                    let color = cube.next().unwrap();
                    match color {
                        "red" if num > game.red => game.red = num,
                        "green" if num > game.green => game.green = num,
                        "blue" if num > game.blue => game.blue = num,
                        _ => (),
                    }
                }
            }
            game
        })
        .map(|game| game.power())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(part1(input), 8);
    }

    #[test]
    fn test_part2() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(part2(input), 2286);
    }
}
