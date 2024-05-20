use anyhow::anyhow;
use std::ops::{Add, AddAssign, Deref, Mul};

pub struct Vector<T> {
    data: Vec<T>,
}

impl<T> Deref for Vector<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> Vector<T> {
    pub fn new(data: impl Into<Vec<T>>) -> Self {
        Self { data: data.into() }
    }
}

pub fn dot_product<T>(a: Vector<T>, b: Vector<T>) -> anyhow::Result<T>
where
    T: Default + Add<Output = T> + Mul<Output = T> + AddAssign + Copy,
{
    if a.len() != b.len() {
        return Err(anyhow!(
            "The length of a is different from the length of b!!!"
        ));
    }
    let mut dot_sum = T::default();
    for i in 0..a.len() {
        dot_sum += a[i] * b[i];
    }
    Ok(dot_sum)
}
