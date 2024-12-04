use std::fs;

fn main() {
    let data = fs::read_to_string("assets/day_4.txt").unwrap();

    let result_p1 = word_search(&data);

    println!("{}", result_p1);
}

fn word_search(data: &str) -> i32 {
    let mut total = 0;
    let mut chars_in_lines: Vec<Vec<char>> = Vec::new();

    for line in data.lines() {
        chars_in_lines.push(line.chars().collect());
    }

    let line_count = chars_in_lines.len();

    // always the same
    let line_char_count = chars_in_lines[0].len();

    for i in 0..line_count {
        for j in 0..line_char_count {
            if chars_in_lines[i][j] != 'X' {
                continue;
            }

            // horizontal left
            if j + 3 < line_char_count
                && chars_in_lines[i][j + 1] == 'M'
                && chars_in_lines[i][j + 2] == 'A'
                && chars_in_lines[i][j + 3] == 'S'
            {
                total += 1;
            }

            // horizontal right
            if (j as i32) - 3 >= 0
                && chars_in_lines[i][j - 1] == 'M'
                && chars_in_lines[i][j - 2] == 'A'
                && chars_in_lines[i][j - 3] == 'S'
            {
                total += 1;
            }

            if (i as i32) - 3 >= 0 {
                if chars_in_lines[i - 1][j] == 'M'
                    && chars_in_lines[i - 2][j] == 'A'
                    && chars_in_lines[i - 3][j] == 'S'
                {
                    total += 1;
                }

                // diagonal up right
                if j + 3 < line_char_count
                    && chars_in_lines[i - 1][j + 1] == 'M'
                    && chars_in_lines[i - 2][j + 2] == 'A'
                    && chars_in_lines[i - 3][j + 3] == 'S'
                {
                    total += 1;
                }

                // diagonal up left
                if (j as i32) - 3 >= 0
                    && chars_in_lines[i - 1][j - 1] == 'M'
                    && chars_in_lines[i - 2][j - 2] == 'A'
                    && chars_in_lines[i - 3][j - 3] == 'S'
                {
                    total += 1;
                }
            }

            if i + 3 < line_count {
                // vertical down
                if chars_in_lines[i + 1][j] == 'M'
                    && chars_in_lines[i + 2][j] == 'A'
                    && chars_in_lines[i + 3][j] == 'S'
                {
                    total += 1;
                }

                // diagonal down right
                if j + 3 < line_char_count
                    && chars_in_lines[i + 1][j + 1] == 'M'
                    && chars_in_lines[i + 2][j + 2] == 'A'
                    && chars_in_lines[i + 3][j + 3] == 'S'
                {
                    total += 1;
                }

                // diagonal down left
                if (j as i32) - 3 >= 0
                    && chars_in_lines[i + 1][j - 1] == 'M'
                    && chars_in_lines[i + 2][j - 2] == 'A'
                    && chars_in_lines[i + 3][j - 3] == 'S'
                {
                    total += 1;
                }
            }
        }
    }

    total
}
