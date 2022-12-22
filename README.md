# A crate for dealing with 2D data structures

This crate provides a 2D data structure called [`X`].

## Program Structure
A grid consists of three things:
  - X Axis (A Vector of Vectors)
  - Y Axis (A Vector)
  - Intersection Point (The data stored inside of Y)

# Examples

## Create a grid from a string (or any iterator of vectors).
```
use symetrical_grid::{grid, X, Y};

 let example_grid = "012
                     345
                     678";
 let grid = example_grid
     .lines()
     .map(|x| x.trim().chars().map(|x| u32::from(x) - 48).collect::<Vec<u32>>())
     .collect::<X<u32>>();
 let mut constant_grid: X<u32> = grid![[0,1,2], [3, 4, 5], [6, 7, 8]];
 assert_eq!(grid, constant_grid);

```
You can do the same thing with a row
```
use symetrical_grid::Y;
let mut control: Y<char> = Y::new();
control.push('h');
control.push('e');
control.push('l');
control.push('l');
control.push('o');
let input = "hello";
let test = input.chars().collect::<Y<char>>();
assert_eq!(control, test);
```
