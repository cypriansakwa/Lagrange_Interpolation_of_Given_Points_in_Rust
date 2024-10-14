## Overview 
This Rust program implements the Lagrange interpolation method to estimate the value of a function at a specific point, given a set of known points.
## Features
- **Lagrange Interpolation:** Interpolates a value for a given $x$ based on a set of points.
- **Flexible Input:** You can modify the points and the target $x$ value to compute the interpolated result for different cases.
- **Simple and Efficient:** Implements the Lagrange formula in a straightforward and efficient manner using Rust.
## How it Works
- Given a set of $n + 1$ distinct points $(x_0, y_0), (x_1, y_1), \ldots, (x_n, y_n)$, the Lagrange interpolating polynomial $P(x)$ is defined as:
	$$P(x) =\sum_{i=0}^{n} y_i L_i(x)$$
	where $L_i(x)$ are the Lagrange basis polynomials, defined as:
	$$L_i(x) = \prod\limits_{{0 \leq j \leq n}_{j \neq i}} \frac{x - x_j}{x_i - x_j}$$
 - Given a set of known points, the program uses this formula to calculate the value of the interpolated polynomial at any given point $x$.
## Example
- Given the points $(0,4), (−2,1)$, and $(2,3)$, this program computes the interpolated value for any 
$x$ you choose. For example, if $x=-2$, the output will show the value of the polynomial at that point.
## Example Output
>```
>The interpolated value at x = -2 is: 1.0

## Requirements
- Rust installed on your machine. (If Rust is not installed, follow the instructions on the [official Rust website](https://www.rust-lang.org/tools/install) to install it).

## Contributing
  - If you intend to contribute to this project, fork the repository and make a pull request.

## Usage
- To use this code, you can clone or download this repository.
- Compile and run the Rust program using the following command:
>```
>cargo build
>cargo run
## Acknowledgments
- Rust

## Clone the repository:

   ```bash
   git clone Lagrange_Interpolation_of_Given_Points_in_Rust.git
   cd Lagrange_Interpolation_of_Given_Points_in_Rust
