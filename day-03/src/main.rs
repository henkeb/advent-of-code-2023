use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", part1(input));
    // println!("Part 2: {}", part2(input));
}

struct Map {
    height: usize,
    width: usize,
    map: Vec<Vec<char>>,
}

fn read_to_map(input: &str) -> Map {
    let mut map: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        map.push(row);
    }
    Map {
        height: map.len(),
        width: map[0].len(),
        map,
    }
}

// fn find_symbol(c: char) -> bool {
//     c.is_ascii_punctuation() && c != '.'
// }

fn part1(input: &str) -> usize {
    let map = read_to_map(input);

    // let symbols: HashSet<(usize, usize)> = input
    //     .trim()
    //     .lines()
    //     .enumerate()
    //     .flat_map(|(row, line)| {
    //         line.chars()
    //             .enumerate()
    //             .filter_map(move |(col, c)| match find_symbol(c) {
    //                 true => Some((row, col)),
    //                 false => None,
    //             })
    //     })
    //     .collect();

    let mut numbers: Vec<(u32, (usize, usize, usize))> = vec![];
    let mut symbols: HashSet<(usize, usize)> = HashSet::new();

    for (row, line) in input.trim().lines().enumerate() {
        let mut start: u32 = 0;
        for (col, c) in line.chars().enumerate() {
            match c {
                num if num.is_ascii_digit()
                    && map.map[row][col.saturating_sub(1)] == '.'
                    && col != map.width =>
                {
                    start = num.to_digit(10).unwrap();
                }
                num if num.is_ascii_digit() && col + 1 < map.width => {
                    if map.map[row][col + 1] == '.' {
                        let slice = &map.map[row][start as usize..=col];
                        let str: String = slice.iter().collect();
                        println!("{}", str);
                        numbers.push((str.parse().unwrap(), (row, start as usize, col)));
                    }
                }
                num if num.is_ascii_digit() && col + 1 == map.width => {
                    let slice = &map.map[row][start as usize..=col];
                    let str: String = slice.iter().collect();
                    numbers.push((str.parse().unwrap(), (row, start as usize, col)));
                }
                _ if c.is_ascii_punctuation() && c != '.' => {
                    symbols.insert((row, col));
                }
                _ => (),
            }
        }
    }

    for num in numbers {
        println!("{}", num.0);
    }

    // const NEIGHBOURS: [(isize, isize);8] =[
    //     (-1,-1),
    //     (-1,0),
    //     (-1,1),
    //     (0,-1),
    //     (0,1),
    //     (1,-1),
    //     (1,-0),
    //     (1,1)
    // ];

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(part1(input), 4361);
    }

    #[test]
    fn test_part2() {}
}
