use std::{fmt::Display, ops::{Index, IndexMut, Add, AddAssign}};

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

impl<T> Add<&Matrix<T>> for &Matrix<T>
where T: Add<T, Output = T> + Copy {
    type Output = Matrix<T>;

    fn add(self, rhs: &Matrix<T>) -> Self::Output {
        let data: Vec<T> = self.data.iter().zip(rhs.data.iter()).map(|(l, r)| *l + *r).collect();

        Matrix {
            width: self.width,
            height: self.height,
            data,
        }
    }
}

impl<T> AddAssign<&Matrix<T>> for Matrix<T>
where T: Add<T, Output=T> + Copy {
    fn add_assign(&mut self, rhs: &Matrix<T>) {
        self.data = self.data.iter().zip(rhs.data.iter()).map(|(l, r)| *l + *r).collect();
    }
}

// Add matrix to constant
impl<T> Add<T> for &Matrix<T>
where T: Add<T, Output = T> + Copy {
    type Output = Matrix<T>;

    fn add(self, rhs: T) -> Self::Output {
        let data: Vec<T> = self.data.iter().map(|l| *l + rhs).collect();

        Matrix {
            width: self.width,
            height: self.height,
            data,
        }
    }
}

impl<T> AddAssign<T> for Matrix<T>
where T: AddAssign<T> + Copy {
    fn add_assign(&mut self, rhs: T) {
        for x in 0..self.width {
            for y in 0..self.height {
                self[(x, y)] += rhs
            }
        }
    }
}

impl<T: Display> Display for Matrix<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for y in 0..self.height {
            if y > 0 {
                write!(f, " ")?
            }

            write!(f, "[")?;
            for x in 0..self.width {
                write!(f, "{}", self[(x, y)])?;

                if x < self.width - 1 {
                    write!(f, ", ")?
                }
            }

            write!(f, "]")?;

            if y < self.height - 1 {
                write!(f, "\n")?
            }
        }
        write!(f, "]")?;

        Ok(())
    }
}