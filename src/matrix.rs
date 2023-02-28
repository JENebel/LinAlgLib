use std::{fmt::Display, ops::{Index, IndexMut, Add, AddAssign, Mul, MulAssign, Div, DivAssign, Sub, SubAssign}};

#[derive(Clone)]
pub struct Matrix<T> {
    width: usize,
    height: usize,
    data: Vec<T>
}

impl<T> Matrix<T> {
    fn calc_index(&self, x: usize, y: usize) -> usize {
        self.width * y + x
    }

    fn is_vector(&self) -> bool {
        self.height == 1 || self.width == 1
    }
}

impl<T: Default + Clone> Matrix<T> {
    pub fn new_empty(width: usize, height: usize) -> Matrix<T> {
        Matrix {
            width,
            height,
            data: vec![T::default(); width * height],
        }
    }
}

impl<T: Clone> Matrix<T> {
    pub fn new_templated(width: usize, height: usize, template: T) -> Matrix<T> {
        Matrix {
            width,
            height,
            data: vec![template; width * height],
        }
    }

    pub fn row(&self, row: usize) -> Matrix<T> {
        Self {
            width: self.width,
            height: 1,
            data: self.data[self.width * row..self.width * row + self.width].iter().cloned().collect()
        }
    }

    pub fn column(&self, column: usize) -> Matrix<T> {
        let mut new_data = Vec::new();
        for y in 0..self.height {
            new_data.push(self.data[self.calc_index(column, y)].clone())
        }

        Self {
            width: 1,
            height: self.height,
            data: new_data
        }
    }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let index = self.calc_index(index.0, index.1);
        &self.data[index]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let index = self.calc_index(index.0, index.1);
        &mut self.data[index]
    }
}

impl<T: Display + Clone> Display for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str_mat = self.to_string_matrix();

        let mut cell_widths = Vec::new();
        for x in 0..self.width {
            cell_widths.push(str_mat.column(x).data.iter().map(|s| s.len()).max().unwrap())
        }

        let mut total_width: usize = cell_widths.iter().sum();
        total_width += self.width * 2;

        writeln!(f, "╭{:width$}╮", "", width=total_width)?;

        for y in 0..self.height {
            write!(f, "│ ")?;
            for x in 0..self.width {
                if x != 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{:width$}", str_mat[(x, y)], width=cell_widths[x])?;
            }
            write!(f, " │\n")?;
        }

        writeln!(f, "╰{:width$}╯", "", width=total_width)?;

        Ok(())
    }
}

impl<T: Display> Matrix<T> {
    fn to_string_matrix(&self) -> Matrix<String> {
        Matrix {
            width: self.width,
            height: self.height,
            data: self.data.iter().map(|x| format!("{}", x)).collect()
        }
    }
}

/// Operators
macro_rules! implement_matrix_operator {
    ($trait_name: ident, $fn_name: ident, $assign_trait_name: ident, $assign_fn_name: ident,  $operator: tt) => {
        impl<T> $trait_name<&Matrix<T>> for &Matrix<T>
        where T: $trait_name<T, Output = T> + Clone {
            type Output = Matrix<T>;
            fn $fn_name(self, rhs: &Matrix<T>) -> Self::Output {
                // If they are not same size or row/column vectors of same length
                if ((rhs.width != self.width) | (rhs.height != self.height)) &&
                   !(self.is_vector() && rhs.is_vector() && self.data.len() == rhs.data.len()) {
                    panic!("Incompatible matrices: ({}, {}) and ({}, {})", self.width, self.height, rhs.width, rhs.height)
                }
                Matrix {
                    width: self.width,
                    height: self.height,
                    data: self.data.iter().zip(rhs.data.iter()).map(|(l, r)| l.clone() $operator r.clone()).collect()
                }
            }
        }

        impl<T> $trait_name<T> for &Matrix<T>
        where T: $trait_name<T, Output = T> + Clone {
            type Output = Matrix<T>;

            fn $fn_name(self, rhs: T) -> Self::Output {
                self.$fn_name(&Matrix::new_templated(self.width, self.height, rhs))
            }
        }

        impl<T> $assign_trait_name<&Matrix<T>> for Matrix<T>
        where T: $trait_name<T, Output = T>, T: Clone {
            fn $assign_fn_name(&mut self, rhs: &Matrix<T>) {
                self.data = self.$fn_name(rhs).data;
            }
        }

        impl<T> $assign_trait_name<T> for Matrix<T>
        where T: $trait_name<T, Output = T>, T: Clone {
            fn $assign_fn_name(&mut self, rhs: T) {
                self.data = self.$fn_name(rhs).data;
            }
        }
    };
}

implement_matrix_operator!(Add, add, AddAssign, add_assign, +);
implement_matrix_operator!(Sub, sub, SubAssign, sub_assign, -);
implement_matrix_operator!(Mul, mul, MulAssign, mul_assign, *);
implement_matrix_operator!(Div, div, DivAssign, div_assign, /);