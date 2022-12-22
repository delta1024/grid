use super::{Point, Y};
use std::{
    ops::{Deref, DerefMut},
    slice,
};
#[derive(Debug, Clone)]
/// A representation of a 2D data structure.
#[repr(transparent)]
pub struct X<T>(pub(crate) Vec<Y<T>>);

impl<T> Default for X<T>
where
    T: Default + Clone + Into<Point<T>>,
{
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl<T> X<T>
where
    T: Default + Clone + Into<Point<T>>,
{
    /// Returns a new instance of Self
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /** Returns a reference to the value at point.
    # Examples
    ```
    use symetrical_grid::grid;

    let grid = grid![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
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
    # Examples
    ```
    use symetrical_grid::{grid, Point};

    let mut grid = grid![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
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

    /** Returns an iterator visiting all values in each row.
    # Examples
    ```
    use symetrical_grid::grid;
    let grid=  grid![&[1, 2][..], &[3, 4][..]];
    let mut iter = grid.iter();
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&4));
    assert_eq!(iter.next(), None);
    ```
    */
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        let mut iter = self.0.iter();
        let column: slice::Iter<Point<T>> = match iter.next() {
            Some(column) => column.iter(),
            None => [].iter(),
        };
        Iter {
            rows: iter,
            colums: column,
        }
    }
    /** Provides a forward iterator with mutable references.
    # Examples
    ```
    use symetrical_grid::grid;
    let mut grid = grid![[1, 2], [3, 4]];
    let mut iter = grid.iter_mut();
    assert_eq!(iter.next(), Some(&mut 1));
    assert_eq!(iter.next(), Some(&mut 2));
    assert_eq!(iter.next(), Some(&mut 3));
    assert_eq!(iter.next(), Some(&mut 4));
    assert_eq!(iter.next(), None);

    ```
    */
    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, T> {
        let mut iter = self.0.iter_mut();
        let column: slice::IterMut<Point<T>> = match iter.next() {
            Some(column) => column.iter_mut(),
            None => [].iter_mut(),
        };
        IterMut {
            rows: iter,
            columns: column,
        }
    }
}
impl<T> X<T>
where
    T: Default + Clone + Into<Point<T>>,
{
    /** Adds a new row to the grid.
    # Examples
    ```
    # fn main() {
        use symetrical_grid::X;
        let mut grid: X<i32> = X::new();
        let mut x = grid.len();
        assert_eq!(x, 0);
        grid.add_row();
        x = grid.len();
        assert_eq!(x, 1);
    # }
    ```
    */
    pub fn add_row(&mut self) {
        self.0.push(Y::new());
        let grid_depth = self.0[0].len();
        let len = self.0.len() - 1;
        self.0[len].0.resize(grid_depth, T::default().into())
    }
    /** Adds column to X.
    # Examples
    ```
    # use symetrical_grid::{X, Point};
    # fn main() {
        let mut grid = X::new();
        grid.push_point(3, 4, 7);
        grid.add_column();
        assert_eq!(grid[3][5], Point::from(u32::default()));
    # }
    ```
    */
    pub fn add_column(&mut self) {
        if self.0.is_empty() {
            self.0.push(Y::new())
        }
        for row in &mut self.0[..] {
            row.0.push(T::default().into())
        }
    }

    /** Places `value` at `point`.
    Point is zero indexed.

    X will be expanded to accomidate point location if neseccary.

    # Examples
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
        if self.0.is_empty() {
            self.0.push(Y::new());
        }
        if self.0[0].len() <= y {
            for _ in self.0[0].len()..=y {
                self.add_column();
            }
        }
        if self.0.len() <= x {
            for _ in self.0.len()..=x {
                self.add_row();
            }
        }
        self.0[x][y] = value.into();
    }
    /** Adds a row to the grid.
    # Examples
    ```
    use symetrical_grid::{X, Y};

    fn main() {
        let mut grid: X<i32> = X::new();
        grid.add_row();
        assert_eq!(grid[0], Y::new());
    }
    ```
    */
    #[inline]
    pub fn add_row_no_resize(&mut self) {
        self.0.push(Y::new())
    }

    /** Pops the last row from the grid
    # Examples
    ```
    # use symetrical_grid::{X, Y};
    # fn main() {
        let mut grid: X<u32> = X::new();
        grid.add_row();
        let row: Y<u32> = Y::new();
        assert_eq!(grid.pop_row(), Some(row));
    # }
    ```
    */
    #[inline]
    pub fn pop_row(&mut self) -> Option<Y<T>> {
        self.0.pop()
    }

    /// Resizes `X` in place so that `len` is equal to `new_len`.
    #[inline]
    pub fn resize(&mut self, new_len: usize) {
        self.0.resize(new_len, Y::new());
    }
    /** Pushes `row` to the end of the grid
    # Examples
    ```
    use symetrical_grid::{X, Y, Point};
    let y= Y::from(&([1,2,3])[..]);
    let mut x= X::new();
    x.push_row(y);
    assert_eq!(x[0][1], Point::from(2));
    ```
     */
    #[inline]
    pub fn push_row(&mut self, row: Y<T>) {
        self.0.push(row);
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }
    pub fn with_size(x: usize, y: usize) -> Self {
        let mut y_vec = Y::with_capacity(y);
        y_vec.resize(y, T::default());
        let mut x_vec = X::with_capacity(x);
        x_vec.0.resize(x, y_vec);
        x_vec
    }
}
impl<A> FromIterator<Vec<A>> for X<A> {
    fn from_iter<T: IntoIterator<Item = Vec<A>>>(iter: T) -> Self {
        Self(
            iter.into_iter()
                .map(|x| x.into_iter().collect::<Y<A>>())
                .collect::<Vec<Y<A>>>(),
        )
    }
}

impl<T> Deref for X<T> {
    type Target = [Y<T>];
    fn deref(&self) -> &Self::Target {
        &self.0[..]
    }
}

impl<T> DerefMut for X<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0[..]
    }
}

impl<T> PartialEq for X<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> From<&[&[T]]> for X<T>
where
    T: Clone + Default,
{
    fn from(c: &[&[T]]) -> Self {
        let mut x = X::new();
        for y in c {
            x.push_row(Y::from(*y));
        }
        x
    }
}
/// An iterator over each item in each row.
pub struct Iter<'a, T: 'a> {
    rows: slice::Iter<'a, Y<T>>,
    colums: slice::Iter<'a, Point<T>>,
}
impl<'a, T: 'a> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.colums.next() {
            Some(n) => Some(n),
            None => match self.rows.next() {
                Some(n) => {
                    self.colums = n.iter();
                    self.next()
                }
                None => None,
            },
        }
    }
}
/// A mutable iterator over each item in each row.
pub struct IterMut<'a, T: 'a> {
    rows: slice::IterMut<'a, Y<T>>,
    columns: slice::IterMut<'a, Point<T>>,
}

impl<'a, T: 'a> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.columns.next() {
            Some(n) => Some(n),
            None => match self.rows.next() {
                Some(n) => {
                    self.columns = n.iter_mut();
                    self.next()
                }
                None => None,
            },
        }
    }
}
#[macro_export]
/** Creates a symetrical grid from given input
# Examples
```
use symetrical_grid::{X, Y, grid};

let n = grid![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
let mut control = grid![];
let one = Y::from(&[1, 2, 3][..]);
let two = Y::from(&[4, 5, 6][..]);
let three = Y::from(&[7, 8, 9][..]);
control.push_row(one);
control.push_row(two);
control.push_row(three);
let control = X::from(control);
assert_eq!(n, control);

```
*/
macro_rules! grid {
    ($($grid: expr),*) => {{
        use symetrical_grid::{X, Y};
        let mut x = X::new();
        $(

            let mut o  = Y::new();
            for i in &$grid[..] {
                o.push(i.clone());
            }
            x.push_row(o);
        )*
        x
    }};
    [] => {{
        use symetrical_grid::X;
        X::new()
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
        let mut x_c = X::new();
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
