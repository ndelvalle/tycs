use std::error::Error;
use std::io;
use std::io::Read;

fn read_stdin() -> Result<String, Box<dyn Error>> {
    let mut buf: Vec<u8> = vec![];

    io::stdin().read_to_end(&mut buf)?;

    Ok(String::from_utf8(buf)?)
}

#[derive(Debug, Clone)]
struct Matrix {
    grid: Vec<Vec<u8>>,
    n: usize,
    m: usize,
}

impl Matrix {
    fn get(&self, n: usize, m: usize) -> u8 {
        self.grid.get(m).and_then(|row| row.get(n)).unwrap().clone()
    }

    fn get_visible_tree_count(&self) -> usize {
        let external_count = self.n * 2 + self.m * 2 - 4;

        let matrix = self.clone();
        let internal_count = matrix
            .into_iter()
            .filter(|(n, m)| self.is_visible(*n, *m))
            .collect::<Vec<_>>()
            .len();

        external_count + internal_count
    }

    fn is_visible(&self, n: usize, m: usize) -> bool {
        let item = self.get(n, m);
        let row = self.grid.get(m).unwrap();

        // Horizontal
        let mut is_visible_from_left = true;
        let mut is_visible_from_right = true;
        for (index, current_item) in row.iter().enumerate() {
            if !is_visible_from_left && !is_visible_from_right {
                break;
            }

            let is_left = index < n;
            let is_right = index > n;

            if index == n {
                continue;
            }

            if current_item >= &item {
                if is_left {
                    is_visible_from_left = false;
                } else if is_right {
                    is_visible_from_right = false;
                }
            }
        }
        let is_visible_from_horizontal = is_visible_from_left || is_visible_from_right;
        if is_visible_from_horizontal {
            return true;
        }

        // Vertical
        let mut is_visible_from_top = true;
        let mut is_visible_from_bottom = true;
        let ran = 0..self.m;
        for (index, current_item) in ran.map(|index| self.get(n, index)).enumerate() {
            if !is_visible_from_top && !is_visible_from_bottom {
                break;
            }

            let is_top = index < m;
            let is_bottom = index > m;

            if index == m {
                continue;
            }

            if current_item >= item {
                if is_top {
                    is_visible_from_top = false;
                } else if is_bottom {
                    is_visible_from_bottom = false;
                }
            }
        }
        is_visible_from_top || is_visible_from_bottom
    }
}

impl FromIterator<Vec<u8>> for Matrix {
    fn from_iter<I: IntoIterator<Item = Vec<u8>>>(iter: I) -> Self {
        let grid = iter.into_iter().collect::<Vec<Vec<_>>>();
        let n = grid.len();
        let m = grid.get(0).unwrap().len();
        Matrix { grid, n, m }
    }
}

struct MatrixIntoIterator {
    matrix: Matrix,
    index_n: usize,
    index_m: usize,
}

impl IntoIterator for Matrix {
    type Item = (usize, usize);
    type IntoIter = MatrixIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        MatrixIntoIterator {
            matrix: self,
            index_n: 1,
            index_m: 1,
        }
    }
}

impl Iterator for MatrixIntoIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        // Inclusive limits
        let limit_index_n = self.matrix.n - 2;
        let limit_index_m = self.matrix.m - 2;

        let has_reached_limit_n = self.index_n > limit_index_n;
        if has_reached_limit_n {
            self.index_m += 1;
            self.index_n = 1;
        }

        let has_reached_limit_m = self.index_m > limit_index_m;
        if has_reached_limit_m {
            return None;
        }

        let item = (self.index_n, self.index_m);

        self.index_n += 1;

        Some(item)
    }
}

fn main() {
    let input = read_stdin().unwrap();

    let matrix = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<_>>()
        })
        .collect::<Matrix>();

    let visible_tree_count = matrix.get_visible_tree_count();
    println!("{:?}", visible_tree_count);
}
