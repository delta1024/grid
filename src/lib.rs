//! A library for dealing with 2D data structures
//!
//! This crate provides a 2D data structure called [`Grid`].
//!
//! By default [`Grid`] is symetrical, meaning that the grid has the same number of columns on each row.
//! This can be disabled by using the [`Grid::new_no_symetry`] function.
use std::ops::{Add, Deref, DerefMut, Div, Mul, Sub};

#[derive(Debug, Clone)]
/// A representation of a point on a grid.
pub struct Column<T>(T);
impl<T> Copy for Column<T> where T: Copy {}
impl<T> PartialEq for Column<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<T> Eq for Column<T> where T: Eq {}
impl<T> From<T> for Column<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T> Deref for Column<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Column<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Add for Column<T>
where
    T: Add<Output = T>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl<T> Sub for Column<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl<T> Mul for Column<T>
where
    T: Mul<Output = T>,
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl<T> Div for Column<T>
where
    T: Div<Output = T>,
{
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

#[derive(Debug, Clone)]
/// A representation of a row in a grid.
pub struct Row<T>(Vec<Column<T>>);

impl<T> Row<T>
where
    T: Into<Column<T>>,
{
    /// Returns a new instance of Self.
    pub fn new() -> Self {
        Self(Vec::new())
    }
    /// Calls push on the underling vector.
    pub fn push(&mut self, value: T) {
        self.0.push(value.into())
    }
    /// Calles pop on the underling vector.
    pub fn pop(&mut self) -> Option<T> {
        self.0.pop().map(|x| x.0)
    }
}

impl<T> Deref for Row<T> {
    type Target = [Column<T>];
    fn deref(&self) -> &Self::Target {
        &self.0[..]
    }
}

impl<T> DerefMut for Row<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone)]
/// A representation of a 2D data structure.
pub struct Grid<T> {
    rows: Vec<Row<T>>,
    symetrical: bool,
}

impl<T> Default for Grid<T>
where
    T: Default + Clone + Into<Column<T>>,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Grid<T>
where
    T: Default + Clone + Into<Column<T>>,
{
    /// Returns a new instance of Self
    pub fn new() -> Self {
        Self {
            rows: Vec::new(),
            symetrical: true,
        }
    }
    /// Returns a new asymetrical [`Grid`]
    pub fn new_no_symetry() -> Self {
        Self {
            rows: Vec::new(),
            symetrical: false,
        }
    }
    /** Adds a new row to the grid.
    if Grid is symetrical resizes the new row to match the length of the other rows.*/
    pub fn add_row(&mut self) {
        self.rows.push(Row::new());
        if self.symetrical {
            let len = self.len();
            let grid_depth = self[0].len();
            self[len - 1].0.resize(grid_depth, T::default().into())
        }
    }
    /** Adds column to Grid.
    # Panic
    Panics if called on an asymetrical grid.
    */
    pub fn add_column(&mut self) {
        if !self.symetrical {
            panic!(
                "{} can only be called on a symetrical grid.",
                std::any::type_name::<T>()
            );
        }

        for row in &mut self[..] {
            row.push(T::default())
        }
    }
    /** Places `value` at `point`. Grid will be expanded to accomidate point location if neseccary.
    # Pannic
    Panics if grid is asymerical.
    */
    pub fn push_point(&mut self, point: (usize, usize), value: T) {
        if !self.symetrical {
            panic!(
                "{} can only be called on a symetrical grid.",
                std::any::type_name::<T>()
            );
        }

        let (x, y) = point;
        if self.len() == 0 {
            self.add_row();
        }
        if self[0].len() <= y {
            for _ in self[0].len()..=(y + 1) {
                self.add_column();
            }
        }
        if self.len() <= x {
            for _ in self.len()..=(x + 1) {
                self.add_row();
            }
        }
        self[x][y] = value.into();
    }
}

impl<T> Deref for Grid<T> {
    type Target = [Row<T>];
    fn deref(&self) -> &Self::Target {
        &self.rows[..]
    }
}

impl<T> DerefMut for Grid<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.rows[..]
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_point() {
        let mut grid = Grid::new();
        grid.push_point((3, 4), 5);
        assert_eq!(grid[3][4], Column(5));
    }
}
