const SPELLED_DIGITS: [&[u8]; 9] = [
    b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
];

pub fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn part1(input: &[&str]) -> u32 {
    input
        .iter()
        .map(|line| {
            // wrapping subtraction is used to convert the byte to a decimal
            let first_digit = line
                .bytes()
                .find(u8::is_ascii_digit)
                .unwrap()
                .wrapping_sub(b'0');

            // rfind searches beginning from the end of the bytes
            let last_digit = line
                .bytes()
                .rfind(u8::is_ascii_digit)
                .unwrap()
                .wrapping_sub(b'0');

            // a naive approach would be appending both digits and converting them afterwards
            // instead we can multiply the first digit by 10 and add the last digit
            (first_digit * 10 + last_digit) as u32
        })
        .sum()
}

pub fn part2(input: &[&str]) -> usize {
    input
        .iter()
        .map(|line| {
            let mut line = line.as_bytes();

            let first_digit = 'parent: loop {
                // check simple case
                if line[0].is_ascii_digit() {
                    break line[0].wrapping_sub(b'0') as usize;
                }

                // check for spelled out digits
                for (idx, digit) in SPELLED_DIGITS.iter().enumerate() {
                    if line.starts_with(digit) {
                        break 'parent idx + 1;
                    }
                }

                line = &line[1..];
            };

            let last_digit = 'parent: loop {
                // same as above but beginning from the end of the line
                if line[line.len() - 1].is_ascii_digit() {
                    break line[line.len() - 1].wrapping_sub(b'0') as usize;
                }

                for (digit_value, digit) in SPELLED_DIGITS.iter().enumerate() {
                    if line.ends_with(digit) {
                        break 'parent digit_value + 1;
                    }
                }

                line = &line[..line.len() - 1];
            };

            10 * first_digit + last_digit
        })
        .sum()
}
