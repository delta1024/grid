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

    /** Returns a reference to the value at point.
    ```
    use symetrical_grid::grid;

    let grid = grid!([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
    assert_eq!(grid.get_point(1,1), Some(&5));
    assert_eq!(grid.get_point(13, 45), None);
    ```
    */
    pub fn get_point(&self, x: usize, y: usize) -> Option<&T> {
        if x >= self.len() {
            return None;
        }

        if y >= self[x].len() {
            return None;
        }
        Some(&self[x][y].0)
    }

    /** Returns a mutable reference to the value at point.
    ```
    use symetrical_grid::{grid, Point};

    let mut grid = grid!([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
    grid.get_point_mut(1,1).map(|x| *x = 15);
    assert_eq!(grid[1][1], Point::from(15));
    ```
    */
    pub fn get_point_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        if x >= self.len() {
            return None;
        }

        if y >= self[x].len() {
            return None;
        }
        Some(&mut self[x][y].0)
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
        grid.push_point(3, 4, 7);
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
        grid.push_point(3, 4, 5);
        assert_eq!(grid[3][4], Point::from(5));
    # }
    ```
        */
    pub fn push_point(&mut self, x: usize, y: usize, value: T) {
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
    /** Converts a symetrical `X` value into an asymetrical one.
    ```
    # use symetrical_grid::{X, Asymetrical, Symetrical};
    let f: X<i32, Symetrical> = X::new();
    let x: X<i32, Asymetrical> = X::new();
    assert_eq!(x, f.into_asymetrical());

    ```
    */
    #[inline]
    pub fn into_asymetrical(self) -> X<T, Asymetrical> {
        X::from(self)
    }
}
impl<T> X<T, Asymetrical>
where
    T: Default + Into<Point<T>> + Clone,
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

    /// Resizes `X` in place so that `len` is equal to `new_len`.
    pub fn resize(&mut self, new_len: usize) {
        self.rows.resize(new_len, Y::new());
    }
    /** Pushes `row` to the end of the grid
    ```
    use symetrical_grid::{X, Y, Asymetrical, Point};
    let y: Y<u32, Asymetrical> = Y::from(&([1,2,3])[..]);
    let mut x: X<u32, Asymetrical> = X::new();
    x.push_row(y);
    assert_eq!(x[0][1], Point::from(2));
    ```
     */
    pub fn push_row(&mut self, row: Y<T, Asymetrical>) {
        self.rows.push(row);
    }

    /** Converts a asymetrical `X` value to a symetrical one,
    ```
    # use symetrical_grid::{X, Asymetrical, Symetrical};
    let f: X<i32, Asymetrical> = X::new();
    let x: X<i32, Symetrical> = X::new();
    assert_eq!(x, f.into_symetrical());

    ```
    */
    #[inline]
    pub fn into_symetrical(self) -> X<T, Symetrical> {
        X::from(self)
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
    T: Default + Into<Point<T>> + Clone,
{
    fn from(mut other: X<T, Asymetrical>) -> Self {
        let max = other.iter().map(|x| x.len()).max().unwrap_or(0);
        for i in &mut other[..] {
            for _ in i.len()..max {
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

impl<T> From<&[&[T]]> for X<T, Asymetrical>
where
    T: Clone + Default,
{
    fn from(c: &[&[T]]) -> Self {
        let mut x: X<T, Asymetrical> = X::new();
        for y in c {
            x.push_row(Y::from(*y));
        }
        x
    }
}

#[macro_export]
/** Creates a asymetrical grid from the given input
```
use symetrical_grid::{X, Y, Asymetrical, grid_asym};

let n = grid_asym!([&[1,2,3][..], &[4,5][..]]);
let mut control: X<i32, Asymetrical> = X::new();
let one: Y<i32, Asymetrical> = Y::from(&([1, 2, 3])[..]);
let two: Y<i32, Asymetrical> = Y::from(&([4, 5])[..]);
control.push_row(one);
control.push_row(two);
assert_eq!(n, control);
``` */
macro_rules! grid_asym {
    ($grid: expr) => {{
        use symetrical_grid::{Asymetrical, Symetrical, X, Y};
        let mut x: X<_, Asymetrical> = X::new();
        for y in &$grid[..] {
            let mut o: Y<_, Asymetrical> = Y::new();
            for i in &y[..] {
                o.push(i.clone());
            }
            x.push_row(o);
        }
        x
    }};
    () => {{
        use symetrical_grid::{Asymetrical, X};
        let x: X<_, Asymetrical> = X::new();
        x
    }};
}
#[macro_export]
/** Creates a symetrical grid from given input
```
use symetrical_grid::{X, Y, Asymetrical, Symetrical, grid, grid_asym};

let n = grid!([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
let mut control = grid_asym!();
let one: Y<i32, Asymetrical> = Y::from(&[1, 2, 3][..]);
let two: Y<i32, Asymetrical> = Y::from(&[4, 5, 6][..]);
let three: Y<i32, Asymetrical> = Y::from(&[7, 8, 9][..]);
control.push_row(one);
control.push_row(two);
control.push_row(three);
let control: X<i32, Symetrical> = X::from(control);
assert_eq!(n, control);

```
*/
macro_rules! grid {
    ($grid: expr) => {{
        use symetrical_grid::{grid_asym, Symetrical, X};
        let n = grid_asym!($grid);
        let n: X<_, Symetrical> = X::from(n);
        n
    }};
    () => {{
        use symetrical_grid::{Symetrical, X};
        let x: X<_, Symetrical> = X::new();
        x
    }};
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn from_slices() {
        let x = [1, 2, 3];
        let y = [2, 3, 4];
        let t = [&x[..], &y[..]];
        let x = X::from(&t[..]);
        let mut x_c: X<i32, Asymetrical> = X::new();
        x_c.add_row();
        x_c.add_row();
        x_c[0].push(1);
        x_c[0].push(2);
        x_c[0].push(3);
        x_c[1].push(2);
        x_c[1].push(3);
        x_c[1].push(4);
        assert_eq!(x, x_c);
    }
}
