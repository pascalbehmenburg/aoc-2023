use std::{
    char, cmp,
    fs::{self, File},
    io,
    path::Path,
};
use std::{
    collections::HashMap,
    fmt::{self, Display},
};

fn main() -> anyhow::Result<()> {
    day_one()?;
    day_two()
}

fn day_one() -> anyhow::Result<()> {
    let lines = read_lines("resources/one.txt")?;
    let mut result = 0;
    for line in lines {
        let digits: Vec<char> = line?.chars().filter(char::is_ascii_digit).collect();
        let line_value =
            format!("{}{}", digits.first().unwrap(), digits.last().unwrap()).parse::<i32>()?;
        result += line_value;
    }
    println!("Answer [1][*]: {}", result);

    let spelled_digits = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let lines = read_lines("resources/one.txt")?;
    let mut result: usize = 0;
    for line in lines {
        let line = line?;

        let mut idx_to_digit: HashMap<usize, usize> = HashMap::new();

        for (digit_value, spelled_digit) in spelled_digits.iter().enumerate() {
            let indeces: Vec<_> = line.match_indices(spelled_digit).collect();
            for i in indeces {
                idx_to_digit.insert(i.0, digit_value + 1);
            }
        }

        for (idx, c) in line.chars().enumerate() {
            if char::is_ascii_digit(&c) {
                idx_to_digit.insert(idx, c.to_digit(10).expect("not a digit") as usize);
            }
        }

        let minimum_idx = idx_to_digit.keys().min().expect("no minimum index");
        let maximum_idx = idx_to_digit.keys().max().expect("no maximum index");

        let min_num = idx_to_digit.get(minimum_idx).expect("no minimum value");
        let max_num = idx_to_digit.get(maximum_idx).expect("no maximum value");

        result += format!("{}{}", min_num, max_num)
            .parse::<usize>()
            .expect("not a number")
    }
    println!("Answer [1][**]: {}", result);

    Ok(())
}

fn day_two() -> anyhow::Result<()> {
    let lines = read_lines("resources/two.txt")?;

    let red_limit = 12;
    let green_limit = 13;
    let blue_limit = 14;

    let mut answer_part_one: usize = 0;
    let mut answer_part_two: usize = 0;

    for game_line in lines {
        let game_line = game_line?;

        let split: Vec<&str> = game_line.split(':').collect();

        let game_id = split[0]
            .chars()
            .filter(char::is_ascii_digit)
            .collect::<String>()
            .parse::<usize>()?;

        let sets: Vec<&str> = split[1].split(';').collect();
        let mut valid_game: bool = true;
        let mut min_rgb: [usize; 3] = [0, 0, 0];

        'set_loop: for set in sets {
            let colors: Vec<&str> = set.split(',').collect();

            for color in colors {
                let color_pair: Vec<&str> = color.trim().split(' ').collect();
                let count = color_pair[0].parse::<usize>()?;
                let color = color_pair[1];

                if valid_game {
                    valid_game = match color {
                        "red" => count <= red_limit,
                        "green" => count <= green_limit,
                        "blue" => count <= blue_limit,
                        _ => false,
                    };
                }

                match color {
                    "red" => min_rgb[0] = cmp::max(min_rgb[0], count),
                    "green" => min_rgb[1] = cmp::max(min_rgb[1], count),
                    "blue" => min_rgb[2] = cmp::max(min_rgb[2], count),
                    _ => (),
                };
            }
        }

        if valid_game {
            answer_part_one += game_id;
        }

        answer_part_two += min_rgb[0] * min_rgb[1] * min_rgb[2];
    }
    println!("Answer [2][*]: {}", answer_part_one);
    println!("Answer [2][**]: {}", answer_part_two);
    Ok(())
}

fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>> {
    Ok(io::BufRead::lines(io::BufReader::new(File::open(
        filename,
    )?)))
}
