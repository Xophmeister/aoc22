use std::io;

type Tree = isize; // signed because we use -1 as a default
type Row = Vec<Tree>;
type Column = Row;
type Matrix = Vec<Row>;

pub struct Forest(Matrix);

/// Decode character code to height
fn decode(c: char) -> Tree {
    (c as Tree) - ('0' as Tree)
}

/// Return a slice of a slice up to (and including) elements that are less than, or equal to some
/// value (which can't be done with Iterator::take_until); or the whole slice, if not found
fn take_until_obscured<T: PartialOrd>(needle: T, haystack: &[T]) -> &[T] {
    let index = haystack.iter().position(|element| *element >= needle);

    match index {
        None => haystack,
        Some(index) => &haystack[..index + 1],
    }
}

impl Forest {
    pub fn parse() -> Result<Self, io::Error> {
        let stdin = io::stdin();
        let mut line = String::new();

        let mut trees: Matrix = Vec::new();

        while stdin.read_line(&mut line)? != 0 {
            let row: Row = line.trim().chars().map(decode).collect();
            trees.push(row);

            line.clear()
        }

        Ok(Forest(trees))
    }

    fn rows(&self) -> &Matrix {
        &self.0
    }

    fn row(&self, j: usize) -> &Row {
        &self.rows()[j]
    }

    fn column(&self, i: usize) -> Column {
        // NOTE This is grossly inefficient; better to compute the columns once, at parse time.
        self.rows().iter().map(|row| row[i]).collect::<Column>()
    }

    fn tree(&self, i: usize, j: usize) -> Tree {
        self.rows()[j][i]
    }

    fn orthogonal(&self, i: usize, j: usize) -> [Vec<Tree>; 4] {
        let column = self.column(i);

        let north = &column[..j];
        let south = &column[j + 1..];
        let east = &self.row(j)[i + 1..];
        let west = &self.row(j)[..i];

        // NOTE north and west are reversed, so they radiate outwards from (i, j)
        // This also leads to unnecessary allocations; better to tag their read direction with an
        // enum (e.g., Forward(&[T]) and Reverse(&[T])
        [
            {
                let mut north = north.to_vec();
                north.reverse();
                north
            },
            south.to_vec(),
            east.to_vec(),
            {
                let mut west = west.to_vec();
                west.reverse();
                west
            },
        ]
    }

    fn size(&self) -> (usize, usize) {
        (self.rows()[0].len(), self.rows().len())
    }

    // Part 1

    fn is_visible(&self, i: usize, j: usize) -> bool {
        let height = self.tree(i, j);

        self.orthogonal(i, j)
            .iter()
            .map(|direction| direction.iter().max().map_or(-1, |height| *height))
            .any(|tallest| tallest < height)
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

    // Part 2

    fn scenery(&self, i: usize, j: usize) -> u32 {
        let height = self.tree(i, j);

        self.orthogonal(i, j)
            .iter()
            .map(|direction| take_until_obscured(height, direction).len() as u32)
            .product()
    }

    pub fn scenic_score(&self) -> u32 {
        let (width, height) = self.size();
        let mut scenery = 0;

        // NOTE Trees around the edge have a scenery score of zero, so we skip them
        for j in 1..height - 1 {
            for i in 1..width - 1 {
                scenery = scenery.max(self.scenery(i, j));
            }
        }

        scenery
    }
}
