use rand::prelude::*;

pub struct Slice {
    rows: usize,
    columns: usize,
    data: Vec<bool>,
    at: usize,
}

pub struct Cell {
    row: usize,
    column: usize,
    value: bool,
}

impl Cell {
    pub fn row(&self) -> f32 {
        self.row as f32
    }

    pub fn column(&self) -> f32 {
        self.column as f32
    }

    pub fn value(&self) -> bool {
        self.value
    }
}

impl Slice {
    pub fn new(rows: usize, columns: usize) -> Slice {
        let mut rng = rand::thread_rng();

        Slice {
            at: 0,
            rows,
            columns,
            data: vec![false; rows * columns].iter().map(|_| rng.gen::<bool>()).collect(),
        }
    }
}

impl Iterator for Slice {
    type Item = Cell;

    fn next(&mut self) -> Option<Self::Item> {
        if self.at >= self.rows * self.columns {
            None
        } else {
            let row = self.at / self.columns;
            let column = self.at % self.columns;
            let value = self.data[row * self.columns + column];

            Some(Cell { row, column, value })
        }
    }
}