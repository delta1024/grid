use crate::Symetrical;

use super::{Asymetrical, Mode, Point};
use std::ops::{Deref, DerefMut};
#[derive(Debug, Clone)]
/// A representation of a row in a grid.
pub struct Y<T, M: Mode> {
    pub(crate) points: Vec<Point<T>>,
    _mode: M,
}
impl<T, U> PartialEq for Y<T, U>
where
    T: PartialEq,
    U: Mode + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.points == other.points
    }
}
impl<T, U> Eq for Y<T, U>
where
    T: Eq,
    U: Mode + Eq,
{
}
impl<U, T> Y<T, U>
where
    T: Into<Point<T>>,
    U: Mode,
{
    /// Returns a new instance of Self.
    pub fn new() -> Self {
        Self {
            points: Vec::new(),
            _mode: U::default(),
        }
    }
}
impl<U, T> Default for Y<T, U>
where
    U: Mode,
{
    fn default() -> Self {
        Self::new()
    }
}
impl<T> Y<T, Symetrical>
where
    T: Into<Point<T>> + Clone,
{
    /** Converts a symetrical Y to an asymetrical Y.
    ```
    # use symetrical_grid::{Y, Asymetrical, Symetrical};

    let f: Y<i32, Symetrical> = Y::new();
    let s: Y<i32, Asymetrical> = Y::new();
    assert_eq!(s, f.into_asymetrical());
    ```
    */
    #[inline]
    pub fn into_asymetrical(self) -> Y<T, Asymetrical> {
        Y::from(self)
    }
}
impl<T> Y<T, Asymetrical>
where
    T: Into<Point<T>> + Clone,
{
    /** Calls push on the underling vector.

    # Example
    ```
    # use symetrical_grid::{X, Point, Asymetrical, Y};
    # fn main() {
        let mut grid: X<i32, Asymetrical> = X::new();
        grid.add_row();
        grid[0].push(3);
        assert_eq!(grid[0][0], Point::from(3));
    # }
    */
    pub fn push(&mut self, value: T) {
        self.points.push(value.into())
    }
    /** Calles pop on the underling vector.
    # Example
    ```
    # use symetrical_grid::{Y, Asymetrical};
    # fn main() {
        let mut y: Y<u32, Asymetrical> = Y::new();
        y.push(3);
        assert_eq!(y.pop(), Some(3));
    # }
    ```
    */
    pub fn pop(&mut self) -> Option<T> {
        self.points.pop().map(|x| x.unwrap())
    }
    /// Resizes `Y` in place so that `len` is equal to `new_len`.
    pub fn resize(&mut self, new_len: usize, value: T) {
        self.points.resize(new_len, value.into())
    }
    /** Converts an asymetrical Y to a symetrical Y.
    ```
    # use symetrical_grid::{Y, Asymetrical, Symetrical};

    let f: Y<i32, Asymetrical> = Y::new();
    let s: Y<i32, Symetrical> = Y::new();
    assert_eq!(s, f.into_symetrical());
    ```
    */
    #[inline]
    pub fn into_symetrical(self) -> Y<T, Symetrical> {
        Y::from(self)
    }
}

impl<T, U> Deref for Y<T, U>
where
    U: Mode,
{
    type Target = [Point<T>];
    fn deref(&self) -> &Self::Target {
        &self.points[..]
    }
}

impl<T, U> DerefMut for Y<T, U>
where
    U: Mode,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.points[..]
    }
}

impl<A, M: Mode + Default> FromIterator<A> for Y<A, M> {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        Self {
            points: iter
                .into_iter()
                .map(|x| Point::from(x))
                .collect::<Vec<Point<A>>>(),
            _mode: M::default(),
        }
    }
}

impl<T> From<Y<T, Asymetrical>> for Y<T, Symetrical> {
    fn from(other: Y<T, Asymetrical>) -> Self {
        Self {
            points: other.points,
            _mode: Symetrical,
        }
    }
}

impl<T> From<Y<T, Symetrical>> for Y<T, Asymetrical> {
    fn from(other: Y<T, Symetrical>) -> Self {
        Self {
            points: other.points,
            _mode: Asymetrical,
        }
    }
}

impl<T> From<&[T]> for Y<T, Asymetrical>
where
    T: Clone,
{
    fn from(v: &[T]) -> Self {
        Self {
            points: v
                .iter()
                .map(|x| Point::from(x.clone()))
                .collect::<Vec<Point<T>>>(),
            _mode: Asymetrical,
        }
    }
}

#[macro_export]
/** Creates a new asymetrical row.
```
use symetrical_grid::{row_asym, Y, Asymetrical};

let row: Y<i32, Asymetrical> = row_asym!([1, 2, 3]);
let control: Y<i32, Asymetrical> = Y::from(&[1,2,3][..]);
assert_eq!(row, control);
```
*/
macro_rules! row_asym {
    ($row: expr) => {{
        use symetrical_grid::{Asymetrical, Y};
        let mut n: Y<_, Asymetrical> = Y::new();
        for val in &$row[..] {
            n.push(val.clone());
        }
        n
    }};
    () => {
        todo!()
    };
}
#[macro_export]
/** Creates a new symetrical row.
```
use symetrical_grid::{row, Y, Symetrical};

let row: Y<i32, Symetrical> = row!([1, 2, 3]);
let control: Y<i32, Symetrical> = Y::from(&[1,2,3][..]).into_symetrical();
assert_eq!(row, control);
```
*/
macro_rules! row {
    ($row: expr) => {{
        use symetrical_grid::row_asym;
        let row = row_asym!($row);
        row.into_symetrical()
    }};
    () => {
        row_asym!().into_symetrical()
    };
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn from_slices() {
        let control = [1, 2, 3];
        let y = Y::from(&control[..]);
        let mut control: Y<i32, Asymetrical> = Y::new();
        control.push(1);
        control.push(2);
        control.push(3);
        assert_eq!(control, y);
    }
}
