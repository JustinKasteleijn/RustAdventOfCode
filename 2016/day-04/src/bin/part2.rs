fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

struct Grid {
    adj_matrix: Vec<Vec<char>>,
}

impl Grid {
    fn from_string(input: &str) -> Grid {
        let grid: Vec<Vec<char>> = input
            .lines()
            .map(|line| line.chars()
                .collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();
        return Grid { adj_matrix: grid };
    }

    fn check_x_mas(&self) -> u32 {
        let mut count = 0;
        let rows = self.adj_matrix.len();
        let cols = self.adj_matrix[0].len();

        for i in 0..rows.saturating_sub(2) {
            for j in 0..cols.saturating_sub(2) {
                if self.is_x_mas(i, j) {
                    count += 1;
                }
            }
        }

        return count;
    }

    fn is_x_mas(&self, row: usize, col: usize) -> bool {
        let rows = self.adj_matrix.len();
        let cols = self.adj_matrix[0].len();

        // Check bounds before accessing the grid
        if row + 2 >= rows || col + 2 >= cols {
            return false;
        }

        let top_left = self.adj_matrix[row][col];
        let center = self.adj_matrix[row + 1][col + 1];
        let bottom_right = self.adj_matrix[row + 2][col + 2];

        let bottom_left = self.adj_matrix[row + 2][col];
        let top_right = self.adj_matrix[row][col + 2];

        let valid_diagonal1 = (top_left == 'M' && center == 'A' && bottom_right == 'S')
            || (top_left == 'S' && center == 'A' && bottom_right == 'M');
        let valid_diagonal2 = (bottom_left == 'M' && center == 'A' && top_right == 'S')
            || (bottom_left == 'S' && center == 'A' && top_right == 'M');

        return valid_diagonal1 && valid_diagonal2;
    }
}

fn part2(input: &str) -> u32 {
    let grid = Grid::from_string(input);
    return grid.check_x_mas();
}

#[cfg(test)]
mod tests {
    use crate::part2;

    #[test]
    fn part2_test() {
        let result = part2(".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........",
        );
        assert_eq!(result, 9);
    }
}
