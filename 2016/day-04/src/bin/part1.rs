fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

struct Grid {
    adj_matrix: Vec<Vec<char>>,
}

impl Grid {
    fn from_string(input: &str) -> Grid {
        let grid: Vec<Vec<char>> = input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        return Grid { adj_matrix: grid };
    }

    fn check_horizontal(&self) -> u32 {
        let mut res = 0;
        let rows = self.adj_matrix.len();
        let cols = self.adj_matrix[0].len();

        for row in 0..rows {
            for col in 0..cols - 3 {
                let window = &self.adj_matrix[row][col..col + 4];
                if self.is_match(window) {
                    res += 1;
                }
            }
        }
        return res;
    }

    fn check_vertical(&self) -> u32 {
        let mut res = 0;
        let rows = self.adj_matrix.len();
        let cols = self.adj_matrix[0].len();

        for col in 0..cols {
            for row in 0..rows - 3 {
                if self.is_match_vertical(row, col) {
                    res += 1;
                }
            }
        }
        return res;
    }

    fn check_diagonal(&self) -> u32 {
        let mut res = 0;
        let rows = self.adj_matrix.len();
        let cols = self.adj_matrix[0].len();

        for i in 0..rows - 3 {
            for j in 0..cols - 3 {
                if self.is_match_diagonal(i, j, 1, 1) {
                    res += 1;
                }
                if self.is_match_diagonal(i + 3, j, -1, 1) {
                    res += 1;
                }
            }
        }
        return res;
    }

    fn is_match(&self, sequence: &[char]) -> bool {
        return sequence == ['X', 'M', 'A', 'S']
            || sequence == ['S', 'A', 'M', 'X'];
    }

    fn is_match_vertical(&self, row: usize, col: usize) -> bool {
        return self.is_match(&[
            self.adj_matrix[row][col],
            self.adj_matrix[row + 1][col],
            self.adj_matrix[row + 2][col],
            self.adj_matrix[row + 3][col],
        ]);
    }

    fn is_match_diagonal(&self, row: usize, col: usize, dr: isize, dc: isize) -> bool {
        return self.is_match(&[
            self.adj_matrix[row][col],
            self.adj_matrix[(row as isize + dr) as usize][(col as isize + dc) as usize],
            self.adj_matrix[(row as isize + 2 * dr) as usize][(col as isize + 2 * dc) as usize],
            self.adj_matrix[(row as isize + 3 * dr) as usize][(col as isize + 3 * dc) as usize],
        ]);
    }
}

fn part1(input: &str) -> u32 {
    let grid = Grid::from_string(input);
    return grid.check_horizontal()
        + grid.check_vertical()
        + grid.check_diagonal();
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn part1_test() {
        let result = part1("MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX");
        assert_eq!(result, 18);
    }
}
