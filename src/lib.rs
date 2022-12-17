/*! A library for dealing with 2D data structures

This crate provides a 2D data structure called [`X`].


# Examples

## Create a grid from a string (or any iterator of vectors).
```
use grid::{X, Y, Symetrical};

 let example_grid = "012
                     345
                     678";
 let grid = example_grid
     .lines()
     .map(|x| x.trim().chars().map(|x| u32::from(x) - 48).collect::<Vec<u32>>())
     .collect::<X<u32, Symetrical>>();
 let mut constant_grid: X<u32, Symetrical> = X::new();
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

```
You can do the same thing with a row
```
 use grid::{Y, Asymetrical};
let mut control: Y<char, Asymetrical> = Y::new();
control.push('h');
control.push('e');
control.push('l');
control.push('l');
control.push('o');
let input = "hello";
let test = input.chars().collect::<Y<char, Asymetrical>>();
assert_eq!(control, test);
```
*/

mod point;
mod x;
mod y;

pub use mode::*;
#[doc(inline)]
pub use point::Point;
#[doc(inline)]
pub use x::X;
#[doc(inline)]
pub use y::Y;

mod mode {
    /// The mode of the grid.
    pub trait Mode: Default + Copy + Clone + PartialEq + Eq {}

    #[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
    /// Denotes a symetrical grid.
    pub struct Symetrical;
    impl Mode for Symetrical {}
    #[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
    /// Denotes a asymetrical grid.
    pub struct Asymetrical;
    impl Mode for Asymetrical {}
}
