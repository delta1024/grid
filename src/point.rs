use std::ops::{Add, Deref, DerefMut, Div, Mul, Sub};
#[derive(Debug, Clone)]
/// A representation of a point on a grid.
pub struct Point<T>(pub(crate) T);
impl<T> Point<T> {
    pub fn unwrap(self) -> T {
        self.0
    }
}
impl<T> Copy for Point<T> where T: Copy {}
impl<T> PartialEq for Point<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<T> Eq for Point<T> where T: Eq {}
impl<T> From<T> for Point<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T> Deref for Point<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Point<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Add for Point<T>
where
    T: Add<Output = T>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl<T> Sub for Point<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl<T> Mul for Point<T>
where
    T: Mul<Output = T>,
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl<T> Div for Point<T>
where
    T: Div<Output = T>,
{
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}
