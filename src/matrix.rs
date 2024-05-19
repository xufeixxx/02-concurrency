use std::{
    fmt,
    ops::{Add, AddAssign, Mul},
};

use anyhow::{anyhow, Result};

#[warn(dead_code)]
pub struct Matrix<T> {
    data: Vec<T>,
    row: usize,
    col: usize,
}

#[allow(dead_code)]
pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> Result<Matrix<T>>
where
    T: Add<Output = T> + Mul<Output = T> + AddAssign + Copy + Default,
{
    if a.col != b.row {
        return Err(anyhow!("Matrix multiply error: a.col != b.row"));
    }
    let mut data: Vec<T> = vec![T::default(); a.row * b.col];
    for i in 0..a.row {
        for j in 0..b.col {
            for k in 0..a.col {
                data[i * b.col + j] += a.data[i * a.col + k] * b.data[k * b.col + j];
            }
        }
    }
    Ok(Matrix {
        data,
        row: a.row,
        col: b.col,
    })
}

#[allow(dead_code)]
impl<T> Matrix<T> {
    pub fn new(data: impl Into<Vec<T>>, row: usize, col: usize) -> Self {
        Self {
            data: data.into(),
            row,
            col,
        }
    }
}

impl<T> fmt::Display for Matrix<T>
where
    T: fmt::Display,
{
    // display a 2x3 as {1 2 3, 4 5 6}
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        for i in 0..self.row {
            for j in 0..self.col {
                write!(f, "{}", self.data[i * self.col + j])?;
                if j != self.col - 1 {
                    write!(f, " ")?;
                }
            }

            if i != self.row - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, "}}")?;
        Ok(())
    }
}

impl<T> fmt::Debug for Matrix<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Matrix(row = {}, col = {}, {})",
            self.row, self.col, self
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Ok;

    use super::{multiply, Matrix};

    #[test]
    fn test_matrix_multiply() -> anyhow::Result<()> {
        let a = Matrix::new([1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new([1, 2, 3, 4, 5, 6], 3, 2);
        let c = multiply(&a, &b).unwrap();
        assert_eq!(
            format!("{:?}", c),
            "Matrix(row = 2, col = 2, {22 28, 49 64})"
        );
        Ok(())
    }

    #[test]
    fn test_matrix_display() -> anyhow::Result<()> {
        let a = Matrix::new([1, 2, 3, 4], 2, 2);
        let b = Matrix::new([1, 2, 3, 4], 2, 2);
        let c = multiply(&a, &b).unwrap();
        assert_eq!(
            format!("{:?}", c),
            "Matrix(row = 2, col = 2, {7 10, 15 22})"
        );
        Ok(())
    }
}
