# Symetrical Grid
This crate provides a 2D data structure called `X`.

## Program Structure
A grid consists of three things:
  - X Axis (A Vector of Vectors)
  - Y Axis (A Vector)
  - Intersection Point (The data stored inside of Y)

Each grid implements two states.
  - Symetrical (Each vector in X will have the same lengthh).
  - Asymentrical (Up to the user to determin how they wish to configure the table).

A symetrical grid has the following features:
  - Ability to add a new row to the grid
  - Ability to add a new column to the grid
  - Ability to add a value at (x,y) on the grid and have it adust it's size acordingly.

An asymetrical grid has the following features:
  - Ability for the user to add a new row to the table.
  - Ability for the user to add a new column to a desired row (providing it already exists).
  - Ability for the user to add a new entry to a desired column in a row
    (again providing that the row and column already exist).
  - Ability to remove columns and rows.

Both versions of the grid should have the following behaviour:
  - Ability for the user to index into a posion on the grid
    (providing it already exists) and read/write at that position.
  - Abiility to creat a new instance of a grid.
  - Ability to collect both the x and y axis' from an itterator.


# Examples
# Examples

## Create a grid from a string (or any iterator of vectors).
```
use symetrical_grid::{grid, X, Y, Symetrical};

 let example_grid = "012
                     345
                     678";
 let grid = example_grid
     .lines()
     .map(|x| x.trim().chars().map(|x| u32::from(x) - 48).collect::<Vec<u32>>())
     .collect::<X<u32, Symetrical>>();
 let mut constant_grid: X<u32, Symetrical> = grid![[0,1,2], [3, 4, 5], [6, 7, 8]];
 assert_eq!(grid, constant_grid);

```
You can do the same thing with a row
```
use symetrical_grid::{Y, Asymetrical};
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

# Converting between Asymetrical grids and Symetrical grids
```
use symetrical_grid::{grid, X, Y, Symetrical, Asymetrical};
let control = grid![[0,1,3], [4, 5, 0], [4, 0, 0]];
let mut asym: X<i32, Asymetrical> = X::new();
asym.add_row();
asym.add_row();
asym.add_row();
asym[0].push(0);
asym[0].push(1);
asym[0].push(3);
asym[1].push(4);
asym[1].push(5);
asym[2].push(4);
let sym = asym.into_symetrical();
assert_eq!(sym, control);

```
# Converting between Symetrical grids and Asymetrical grids.
```
use symetrical_grid::{X, Y, Symetrical, Asymetrical, grid};
let test = grid![[1,2,3], [2,3,4]];
let mut control: X<i32, Asymetrical> = X::new();
control.add_row();
control.add_row();
control[0].push(1);
control[0].push(2);
control[0].push(3);
control[1].push(2);
control[1].push(3);
control[1].push(4);
let t = test.into_asymetrical();
assert_eq!(control, t);
```
