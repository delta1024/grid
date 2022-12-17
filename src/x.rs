use super::{Asymetrical, Mode, Point, Symetrical, Y};
use std::ops::{Deref, DerefMut};
#[derive(Debug, Clone)]
/// A representation of a 2D data structure.
pub struct X<T, M: Mode> {
    rows: Vec<Y<T, M>>,
    _mode: M,
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
    # Example
    ```
    # fn main() {
        use symetrical_grid::{X, Symetrical};
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
    # Example
    ```
    # use symetrical_grid::{X, Point, Symetrical};
    # fn main() {
        let mut grid: X<u32, Symetrical> = X::new();
        grid.push_point((3, 4), 7);
        grid.add_column();
        assert_eq!(grid[3][5], Point::from(u32::default()));
    # }
    ```
    */
    pub fn add_column(&mut self) {
        if self.rows.is_empty() {
            self.rows.push(Y::new())
        }
        for row in &mut self.rows[..] {
            row.points.push(T::default().into())
        }
    }

    /** Places `value` at `point`.
    Point is zero indexed.

    X will be expanded to accomidate point location if neseccary.

    # Example
    ```
    # fn main() {
    #   use symetrical_grid::{X, Point};
        let mut grid = X::new();
        grid.push_point((3, 4), 5);
        assert_eq!(grid[3][4], Point::from(5));
    # }
    ```
        */
    pub fn push_point(&mut self, point: (usize, usize), value: T) {
        let (x, y) = point;
        if self.rows.is_empty() {
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
    /** Adds a row to the grid.
    # Example
    ```
    use symetrical_grid::{X, Y, Asymetrical};

    fn main() {
        let mut grid: X<u32, Asymetrical> = X::new();
        grid.add_row();
        assert_eq!(grid[0], Y::new());
    }
    ```
    */
    pub fn add_row(&mut self) {
        self.rows.push(Y::new())
    }

    /** Pops the last row from the grid
    # Example
    ```
    # use symetrical_grid::{X, Y, Asymetrical};
    # fn main() {
        let mut grid: X<u32, Asymetrical> = X::new();
        grid.add_row();
        let row: Y<u32, Asymetrical> = Y::new();
        assert_eq!(grid.pop_row(), Some(row));
    # }
    ```
    */
    pub fn pop_row(&mut self) -> Option<Y<T, Asymetrical>> {
        self.rows.pop()
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

impl<T, U> PartialEq for X<T, U>
where
    T: PartialEq,
    U: Mode,
{
    fn eq(&self, other: &Self) -> bool {
        self.rows == other.rows
    }
}

impl<T> From<X<T, Asymetrical>> for X<T, Symetrical>
where
    T: Default + Into<Point<T>>,
{
    fn from(mut other: X<T, Asymetrical>) -> Self {
        let max = other.iter().map(|x| x.len()).max().unwrap_or(0);
        for i in &mut other[..] {
            for _ in i.len()..=max {
                i.push(T::default());
            }
        }
        Self {
            rows: other
                .rows
                .into_iter()
                .map(Y::from)
                .collect::<Vec<Y<T, Symetrical>>>(),
            _mode: Symetrical,
        }
    }
}

impl<T> From<X<T, Symetrical>> for X<T, Asymetrical>
where
    T: Default + Into<Point<T>>,
{
    fn from(other: X<T, Symetrical>) -> Self {
        Self {
            rows: other
                .rows
                .into_iter()
                .map(Y::from)
                .collect::<Vec<Y<T, Asymetrical>>>(),
            _mode: Asymetrical,
        }
    }
}
