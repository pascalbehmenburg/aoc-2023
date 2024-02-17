pub struct Game(u32, u32, u32);

/// Parse each line into a `Game` which contains the maximum
/// of red, green and blue cubes over all sets of that game
pub fn parse(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .array_chunks::<2>()
                .skip(1)
                .fold(
                    Game(0, 0, 0),
                    |Game(r, g, b), [amount, color]| match color.as_bytes()[0] {
                        b'r' => Game(r.max(amount.parse::<u32>().unwrap()), g, b),
                        b'g' => Game(r, g.max(amount.parse::<u32>().unwrap()), b),
                        b'b' => Game(r, g, b.max(amount.parse::<u32>().unwrap())),
                        _ => unreachable!(),
                    },
                )
        })
        .collect()
}

/// Filter games where the maximum amount of cubes for each color is less than or equal to 12, 13 and 14
/// and sum the indices of these games starting from 1.
pub fn part1(input: &[Game]) -> usize {
    input
        .iter()
        .enumerate()
        .filter_map(|(idx, &Game(r, g, b))| (r <= 12 && g <= 13 && b <= 14).then_some(idx + 1))
        .sum()
}

/// Sum the product of maximum red, green and blue sets of cubes for each game
pub fn part2(input: &[Game]) -> u32 {
    input.iter().map(|Game(r, g, b)| r * g * b).sum()
}
