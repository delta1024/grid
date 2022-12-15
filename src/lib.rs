/*! A library for dealing with 2D data structures

This crate provides a 2D data structure called [`Grid`].
By default [`Grid`] is symetrical, meaning that the grid has the same number of columns on each row.
This can be disabled by using the [`Grid::new_no_symetry`] function.


# Examples

## Create a grid from a string (or any iterator of vectors).
```
use grid::{Grid, Row};

fn main() {

    let example_grid = "012
                        345
                        678";
    let grid = example_grid
        .lines()
        .map(|x| x.trim().chars().map(|x| u32::from(x) - 48).collect::<Vec<u32>>())
        .collect::<Grid<u32>>();
    let mut constant_grid: Grid<u32> = Grid::new();
    constant_grid.push_point((0, 0), 0);
    constant_grid.push_point((0, 1), 1);
    constant_grid.push_point((0, 2), 2);
    constant_grid.push_point((1, 0), 3);
    constant_grid.push_point((1, 1), 4);
    constant_grid.push_point((1, 2), 5);
    constant_grid.push_point((2, 0), 6);
    constant_grid.push_point((2, 1), 7);
    constant_grid.push_point((2, 2), 8);
    assert_eq!(grid, constant_grid);


}
```
You can do the same thing with a row
```
# use grid::Row;
# fn main() {
    let mut control = Row::new();
    control.push('h');
    control.push('e');
    control.push('l');
    control.push('l');
    control.push('o');
    let input = "hello";
    let test = input.chars().collect::<Row<char>>();
    assert_eq!(control, test);
# }
```
*/

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
impl<T> PartialEq for Row<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<T> Eq for Row<T> where T: Eq {}
impl<T> Row<T>
where
    T: Into<Column<T>>,
{
    /// Returns a new instance of Self.
    pub fn new() -> Self {
        Self(Vec::new())
    }
    /** Calls push on the underling vector.

    Grid size will remain consistant if symetry is enabled and you don't manualy extend row 0 in a grid.
    # Example
    ```
    # use grid::{Grid, Column};
    # fn main() {
        let mut grid: Grid<i32> = Grid::new();
        grid.add_row();
        grid[0].push(3);
        assert_eq!(grid[0][0], Column::from(3));
        grid.push_point((0, 0), 2);
        assert_eq!(grid[0][0], Column::from(2));
    # }
    */
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

impl<A> FromIterator<A> for Row<A> {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        Self(
            iter.into_iter()
                .map(|x| Column(x))
                .collect::<Vec<Column<A>>>(),
        )
    }
}

#[derive(Debug, Clone)]
/// A representation of a 2D data structure.
pub struct Grid<T> {
    rows: Vec<Row<T>>,
    symetrical: bool,
}

impl<T> PartialEq for Grid<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.rows == other.rows && self.symetrical == self.symetrical
    }
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
    if Grid is symetrical resizes the new row to match the length of the other rows.
    # Example
    ##
    ```
    # fn main() {
        use grid::Grid;
        let mut grid: Grid<i32> = Grid::new();
        let mut x = grid.len();
        assert_eq!(x, 0);
        grid.add_row();
        x = grid.len();
        assert_eq!(x, 1);
    # }
    ```
    */
    pub fn add_row(&mut self) {
        self.rows.push(Row::new());
        if self.symetrical {
            let len = self.len();
            let grid_depth = self.rows[0].len();
            self.rows[len - 1].0.resize(grid_depth, T::default().into())
        }
    }
    /** Adds column to Grid.
    # Panic
    Panics if called on an asymetrical grid.

    # Example
    ```
    # use grid::{Grid, Column};
    # fn main() {
        let mut grid: Grid<u32> = Grid::new();
        grid.push_point((3, 4), 7);
        grid.add_column();
        assert_eq!(grid[3][5], Column::from(u32::default()));
    # }
    ```
    */
    pub fn add_column(&mut self) {
        if !self.symetrical {
            panic!(
                "{} can only be called on a symetrical grid.",
                std::any::type_name::<T>()
            );
        }

        if self.rows.len() == 0 {
            self.rows.push(Row::new())
        }
        for row in &mut self.rows[..] {
            row.push(T::default())
        }
    }
    /** Places `value` at `point`.
    Point is zero indexed.

    Grid will be expanded to accomidate point location if neseccary.

    # Panic
    Panics if grid is asymerical.

    # Example
    ```
    # fn main() {
    #   use grid::{Grid, Column};
        let mut grid = Grid::new();
        grid.push_point((3, 4), 5);
        assert_eq!(grid[3][4], Column::from(5));
    # }
    ```
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
            self.rows.push(Row::new());
        }
        if self[0].len() <= y {
            for _ in self[0].len()..=y {
                self.add_column();
            }
        }
        if self.len() <= x {
            for _ in self.len()..=x {
                self.add_row();
            }
        }
        self[x][y] = value.into();
    }
}

impl<A> FromIterator<Vec<A>> for Grid<A> {
    fn from_iter<T: IntoIterator<Item = Vec<A>>>(iter: T) -> Self {
        Self {
            rows: iter
                .into_iter()
                .map(|x| x.into_iter().collect::<Row<A>>())
                .collect::<Vec<Row<A>>>(),
            symetrical: false,
        }
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
