use std::io;
#[derive(PartialEq, Clone, Debug)]
pub struct Grid<T> {
    pub h: usize,
    pub w: usize,
    pub data: Vec<T>,
}

/*
 * Grid setters and getters
 */
impl<T> Grid<T>
where
    T: Clone,
{
    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        let w = self.w;
        self.data.get((row) * w + (col))
    }
    pub fn set(&mut self, row: usize, col: usize, data: T) -> Result<(), io::Error> {
        let w = self.w;

        if row < self.h && col < self.w {
            self.data[(row) * w + (col)] = data;
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Index out of bounds",
            ))
        }
    }
    pub fn get_row(&self, row: usize) -> Option<&[T]> {
        if row < self.h {
            Some(&self.data[((row) * self.w)..(row + 1) * self.w])
        } else {
            None
        }
    }
    pub fn set_row(&mut self, row: usize, data: Vec<T>) -> Result<(), io::Error> {
        if row < self.h {
            let row = &mut self.data[((row) * self.w)..row * self.w];
            row.clone_from_slice(&data);
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Index out of bounds",
            ))
        }
    }
    pub fn get_col(&self, col: usize) -> Option<impl Iterator<Item = &T>> {
        if col < self.w {
            Some(self.data.iter().skip(col).step_by(self.w))
        } else {
            None
        }
    }
    pub fn set_col(&mut self, col: usize, data: Vec<T>) -> Result<(), io::Error> {
        if col < self.w {
            let col_indices = (col..self.data.len()).step_by(self.w);
            for (idx, x) in col_indices.zip(data) {
                self.data[idx] = x;
            }
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Index out of bounds",
            ))
        }
    }
    pub fn get_diag(&self) -> Option<impl Iterator<Item = &T>> {
        if self.h == self.w {
            Some(self.data.iter().step_by(self.w + 1))
        } else {
            None
        }
    }
    pub fn set_diag(&mut self, data: Vec<T>) -> Result<(), io::Error> {
        if self.h == self.w {
            let diag_indices = (0..self.data.len()).step_by(self.w + 1);
            for (idx, x) in diag_indices.zip(data) {
                self.data[idx] = x;
            }
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Index out of bounds",
            ))
        }
    }
    pub fn get_anti_diag(&self) -> Option<impl Iterator<Item = &T>> {
        if self.h == self.w {
            Some(
                self.data
                    .iter()
                    .skip(self.w - 1)
                    .step_by(self.w - 1)
                    .take(self.h),
            )
        } else {
            None
        }
    }
    pub fn set_anti_diag(&mut self, data: Vec<T>) -> Result<(), io::Error> {
        if self.h == self.w {
            if self.w == 1 {
                if let Some(x) = data.into_iter().next() {
                    self.data[0] = x;
                }
            } else {
                let w = self.w;
                let anti_indices = ((w - 1)..self.data.len()).step_by(w - 1).take(self.h);
                for (idx, x) in anti_indices.zip(data) {
                    self.data[idx] = x;
                }
            }
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Index out of bounds",
            ))
        }
    }
    /// Returns slices of all the rows in the grid
    ///```
    /// let rows: Vec<&[T]> = grid.rows().collect();
    /// ```
    pub fn rows(&self) -> impl Iterator<Item = &[T]> {
        self.data.chunks(self.w)
    }
    /// Returns iterators of all the columns in the grid
    ///```
    /// let cols: Vec<Vec<&T>> = grid.cols().map(|col| col.collect()).collect();
    /// ```
    pub fn cols(&self) -> impl Iterator<Item = impl Iterator<Item = &T>> {
        (1..=self.w).map(|i| self.get_col(i).unwrap())
    }
}

/*
 * Grid creation functions
 */
impl<T> Grid<T> {
    pub fn new() -> Grid<T> {
        Grid {
            h: 0,
            w: 0,
            data: Vec::new(),
        }
    }
    pub fn from_vec(data: Vec<Vec<T>>) -> Grid<T> {
        let w = data[0].len();
        let h = data.len();
        let mut export = Vec::new();
        for row in data {
            if row.len() == w {
                export.extend(row);
            } else {
                panic!("All rows must have the same width")
            }
        }
        Grid { h, w, data: export }
    }
}
