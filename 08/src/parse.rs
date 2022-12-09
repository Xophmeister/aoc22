use std::io;

use crate::error::ParseError;

type Tree = isize;
type Row = Vec<Tree>;
type Column = Row;
type Matrix = Vec<Row>;

#[derive(Debug)]
pub struct Grid(Matrix);

fn decode(c: char) -> Tree {
    // Decode character code to height
    (c as Tree) - ('0' as Tree)
}

impl Grid {
    pub fn parse() -> Result<Self, ParseError> {
        let stdin = io::stdin();
        let mut line = String::new();

        let mut grid: Matrix = Vec::new();

        while stdin.read_line(&mut line)? != 0 {
            let row: Row = line.trim().chars().map(decode).collect();
            grid.push(row);

            line.clear()
        }

        Ok(Grid(grid))
    }

    fn rows(&self) -> &Matrix {
        &self.0
    }

    fn row(&self, j: usize) -> &Row {
        &self.rows()[j]
    }

    fn column(&self, i: usize) -> Column {
        self.rows().iter().map(|row| row[i]).collect::<Column>()
    }

    fn tree(&self, i: usize, j: usize) -> Tree {
        self.rows()[j][i]
    }

    fn size(&self) -> (usize, usize) {
        (self.rows()[0].len(), self.rows().len())
    }

    fn is_visible(&self, i: usize, j: usize) -> bool {
        let height = self.tree(i, j);
        let column = self.column(i);

        let orthogonal = vec![
            &column[..j],          // North
            &column[j + 1..],      // South
            &self.row(j)[i + 1..], // East
            &self.row(j)[..i],     // West
        ];

        orthogonal
            .iter()
            .map(|direction| direction.iter().max().map_or(-1, |height| *height))
            .any(|direction| direction < height)
    }

    pub fn visible(&self) -> usize {
        let (width, height) = self.size();
        let mut visible = 0;

        for j in 0..height {
            for i in 0..width {
                if self.is_visible(i, j) {
                    visible += 1;
                }
            }
        }

        visible
    }
}
