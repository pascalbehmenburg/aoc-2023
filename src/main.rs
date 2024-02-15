use std::{
    char,
    collections::HashMap,
    fs::{self, File},
    io,
    path::Path,
};

fn main() -> anyhow::Result<()> {
    day_one()
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

fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>> {
    Ok(io::BufRead::lines(io::BufReader::new(File::open(
        filename,
    )?)))
}
