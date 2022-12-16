use super::{Asymetrical, Mode, Point, Symetrical, Y};
use std::ops::{Deref, DerefMut};
#[derive(Debug, Clone)]
/// A representation of a 2D data structure.
pub struct X<T, M: Mode> {
    rows: Vec<Y<T, M>>,
    _mode: M,
}

impl<T, U> PartialEq for X<T, U>
where
    T: PartialEq,
    U: Mode,
{
    fn eq(&self, other: &Self) -> bool {
        self.rows == other.rows
    }
}
impl<T, U> Default for X<T, U>
where
    T: Default + Clone + Into<Point<T>>,
    U: Mode,
{
    fn default() -> Self {
        Self {
            rows: Vec::new(),
            _mode: U::default(),
        }
    }
}

impl<T, U> X<T, U>
where
    T: Default + Clone + Into<Point<T>>,
    U: Mode,
{
    /// Returns a new instance of Self
    pub fn new() -> Self {
        Self {
            rows: Vec::new(),
            _mode: U::default(),
        }
    }
}
impl<T> X<T, Symetrical>
where
    T: Default + Clone + Into<Point<T>>,
{
    /** Adds a new row to the grid.
    if X is symetrical resizes the new row to match the length of the other rows.
    # Example
    ##
    ```
    # fn main() {
        use grid::{X, Symetrical};
        let mut grid: X<i32, Symetrical> = X::new();
        let mut x = grid.len();
        assert_eq!(x, 0);
        grid.add_row();
        x = grid.len();
        assert_eq!(x, 1);
    # }
    ```
    */
    pub fn add_row(&mut self) {
        self.rows.push(Y::new());
        let grid_depth = self.rows[0].len();
        let len = self.rows.len() - 1;
        self.rows[len]
            .points
            .resize(grid_depth, T::default().into())
    }
    /** Adds column to X.
    # Panic
    Panics if called on an asymetrical grid.

    # Example
    ```
    # use grid::{X, Point, Symetrical};
    # fn main() {
        let mut grid: X<u32, Symetrical> = X::new();
        grid.push_point((3, 4), 7);
        grid.add_column();
        assert_eq!(grid[3][5], Point::from(u32::default()));
    # }
    ```
    */
    pub fn add_column(&mut self) {
        if self.rows.len() == 0 {
            self.rows.push(Y::new())
        }
        for row in &mut self.rows[..] {
            row.points.push(T::default().into())
        }
    }

    /** Places `value` at `point`.
    Point is zero indexed.

    X will be expanded to accomidate point location if neseccary.

    # Panic
    Panics if grid is asymerical.

    # Example
    ```
    # fn main() {
    #   use grid::{X, Point};
        let mut grid = X::new();
        grid.push_point((3, 4), 5);
        assert_eq!(grid[3][4], Point::from(5));
    # }
    ```
        */
    pub fn push_point(&mut self, point: (usize, usize), value: T) {
        let (x, y) = point;
        if self.rows.len() == 0 {
            self.rows.push(Y::new());
        }
        if self.rows[0].len() <= y {
            for _ in self.rows[0].len()..=y {
                self.add_column();
            }
        }
        if self.rows.len() <= x {
            for _ in self.rows.len()..=x {
                self.add_row();
            }
        }
        self.rows[x][y] = value.into();
    }
}
impl<T> X<T, Asymetrical>
where
    T: Default + Into<Point<T>>,
{
    pub fn get_vec(&self) -> &Vec<Y<T, Asymetrical>> {
        &self.rows
    }
    pub fn get_vec_mut(&mut self) -> &mut Vec<Y<T, Asymetrical>> {
        &mut self.rows
    }
}
impl<A, U> FromIterator<Vec<A>> for X<A, U>
where
    U: Mode,
{
    fn from_iter<T: IntoIterator<Item = Vec<A>>>(iter: T) -> Self {
        Self {
            rows: iter
                .into_iter()
                .map(|x| x.into_iter().collect::<Y<A, U>>())
                .collect::<Vec<Y<A, U>>>(),
            _mode: U::default(),
        }
    }
}
impl<T: Default + Into<Point<T>>> X<T, Asymetrical> {
    pub fn add_row(&mut self) {
        self.rows.push(Y::new())
    }
}

impl<T, U> Deref for X<T, U>
where
    U: Mode,
{
    type Target = [Y<T, U>];
    fn deref(&self) -> &Self::Target {
        &self.rows[..]
    }
}

impl<T, U> DerefMut for X<T, U>
where
    U: Mode,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.rows[..]
    }
}
