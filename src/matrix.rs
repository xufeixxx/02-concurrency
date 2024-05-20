use std::{
    fmt,
    ops::{Add, AddAssign, Mul},
    sync::mpsc::{self, Sender},
    thread,
};

use anyhow::anyhow;

use crate::{dot_product, Vector};

const NUM_THREADS: usize = 4;

#[warn(dead_code)]
pub struct Matrix<T> {
    data: Vec<T>,
    row: usize,
    col: usize,
}

pub struct MsgInput<T> {
    index: usize,
    row_vec: Vector<T>,
    col_vec: Vector<T>,
}

pub struct MsgOutput<T> {
    index: usize,
    val: T,
}

pub struct Msg<T> {
    input: MsgInput<T>,
    sender: oneshot::Sender<MsgOutput<T>>,
}

impl<T> Mul for Matrix<T>
where
    T: Add<Output = T> + Mul<Output = T> + AddAssign + Copy + Default + Send + 'static,
{
    type Output = Matrix<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        multiply(&self, &rhs).unwrap()
    }
}

// pretend this is a heavy operation, CPU intensive
#[allow(dead_code)]
pub fn multiply<T>(a: &Matrix<T>, b: &Matrix<T>) -> anyhow::Result<Matrix<T>>
where
    T: Add<Output = T> + Mul<Output = T> + AddAssign + Copy + Default + Send + 'static,
{
    if a.col != b.row {
        return Err(anyhow!("Matrix multiply error: a.col != b.row"));
    }

    let senders = (0..NUM_THREADS)
        .map(|_| {
            let (tx, rx) = mpsc::channel::<Msg<T>>();
            thread::spawn(move || {
                for msg in rx {
                    let value = dot_product(msg.input.col_vec, msg.input.row_vec)?;
                    if let Err(e) = msg.sender.send(MsgOutput {
                        index: msg.input.index,
                        val: value,
                    }) {
                        eprintln!("Send error: {:?}", e);
                    }
                }
                Ok::<_, anyhow::Error>(())
            });
            tx
        })
        .collect::<Vec<Sender<Msg<T>>>>();

    let mut data: Vec<T> = vec![T::default(); a.row * b.col];
    let mut receivers = Vec::with_capacity(a.row * b.col);
    for i in 0..a.row {
        for j in 0..b.col {
            let row_vector = Vector::new(&a.data[i * a.col..(i + 1) * a.col]);
            let col_data = b.data[j..]
                .iter()
                .step_by(b.col)
                .copied()
                .collect::<Vec<_>>();
            let col_vector = Vector::new(col_data);
            let idx = i * b.col + j;
            let input = MsgInput::new(idx, row_vector, col_vector);
            let (tx, rx) = oneshot::channel();
            let msg = Msg::new(input, tx);
            if let Err(e) = senders[idx % NUM_THREADS].send(msg) {
                eprintln!("Send error: {:?}", e)
            }
            receivers.push(rx);
        }
    }

    for rx in receivers {
        let output = rx.recv()?;
        data[output.index] = output.val;
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

impl<T> MsgInput<T> {
    pub fn new(idx: usize, row: Vector<T>, col: Vector<T>) -> Self {
        Self {
            index: idx,
            row_vec: row,
            col_vec: col,
        }
    }
}

impl<T> Msg<T> {
    pub fn new(input: MsgInput<T>, sender: oneshot::Sender<MsgOutput<T>>) -> Self {
        Self { input, sender }
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Ok;

    use super::Matrix;

    #[test]
    fn test_matrix_multiply() -> anyhow::Result<()> {
        let a = Matrix::new([1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new([1, 2, 3, 4, 5, 6], 3, 2);
        let c = a * b;
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
        let c = a * b;
        assert_eq!(
            format!("{:?}", c),
            "Matrix(row = 2, col = 2, {7 10, 15 22})"
        );
        Ok(())
    }

    #[test]
    #[should_panic]
    fn test_a_can_not_multiply_b_panic() {
        let a = Matrix::new([1, 2, 3, 4, 5, 6], 2, 3);
        let b = Matrix::new([1, 2, 3, 4], 2, 2);
        let _c = a * b;
    }
}
