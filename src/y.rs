use super::Point;
use std::ops::{Deref, DerefMut};
#[derive(Debug, Clone)]
/// A representation of a row in a grid.
#[repr(transparent)]
pub struct Y<T>(pub(crate) Vec<Point<T>>);
impl<T> PartialEq for Y<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<T> Eq for Y<T> where T: Eq {}
impl<T> Y<T>
where
    T: Into<Point<T>>,
{
    /// Returns a new instance of Self.
    pub fn new() -> Self {
        Self(Vec::new())
    }
}
impl<T> Default for Y<T> {
    fn default() -> Self {
        Self::new()
    }
}
impl<T> Y<T>
where
    T: Into<Point<T>> + Clone,
{
    /** Calls push on the underling vector.

    # Examples
    ```
    # use symetrical_grid::{X, Point, Y};
    # fn main() {
        let mut grid = X::new();
        grid.add_row_no_resize();
        grid[0].push(3);
        assert_eq!(grid[0][0], Point::from(3));
    # }
    */
    #[inline]
    pub fn push(&mut self, value: T) {
        self.0.push(value.into())
    }
    /** Calles pop on the underling vector.
    # Examples
    ```
    # use symetrical_grid::Y;
    # fn main() {
        let mut y = Y::new();
        y.push(3);
        assert_eq!(y.pop(), Some(3));
    # }
    ```
    */
    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        self.0.pop().map(|x| x.unwrap())
    }
    /// Resizes `Y` in place so that `len` is equal to `new_len`.
    #[inline]
    pub fn resize(&mut self, new_len: usize, value: T) {
        self.0.resize(new_len, value.into())
    }
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Y(Vec::with_capacity(capacity))
    }
}

impl<T> Deref for Y<T> {
    type Target = [Point<T>];
    fn deref(&self) -> &Self::Target {
        &self.0[..]
    }
}

impl<T> DerefMut for Y<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0[..]
    }
}

impl<A> FromIterator<A> for Y<A> {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        Self(
            iter.into_iter()
                .map(|x| Point::from(x))
                .collect::<Vec<Point<A>>>(),
        )
    }
}

impl<T> From<&[T]> for Y<T>
where
    T: Clone,
{
    fn from(v: &[T]) -> Self {
        Self(
            v.iter()
                .map(|x| Point::from(x.clone()))
                .collect::<Vec<Point<T>>>(),
        )
    }
}

#[macro_export]
/** Creates a new symetrical row.
```
use symetrical_grid::{row, Y};

let row = row![1, 2, 3];
let control = Y::from(&[1,2,3][..]);
assert_eq!(row, control);
```
*/
macro_rules! row {
    ($($row: expr),*) => {{
        use symetrical_grid::Y;
        let mut n = Y::new();
        $(
            n.push($row.clone());
         )*
        n    }};
    [] => {
        Y::new()
    };
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn from_slices() {
        let control = [1, 2, 3];
        let y = Y::from(&control[..]);
        let mut control = Y::new();
        control.push(1);
        control.push(2);
        control.push(3);
        assert_eq!(control, y);
    }
}
