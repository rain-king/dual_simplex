use core::f64;
use ndarray::{concatenate, Array2 as matrix, Axis};

pub fn original_tableau(maximize: bool, c: &matrix<f64>, a: &matrix<f64>, b: &&matrix<f64>) -> matrix<f64> {
	let tableau_bottom = get_tableu_bottom(a, b);

	let tableau_top = concatenate![
		Axis(1),
		if maximize {
			-c.clone()
		} else {
			c.clone()
		},
		matrix::zeros((1, tableau_bottom.ncols() - c.ncols()))
	];
	concatenate![Axis(0), tableau_top, tableau_bottom]
}

pub fn initialize(maximize: bool, c: &matrix<f64>, a: &matrix<f64>, b: &matrix<f64>) -> matrix<f64> {
	let tableau_bottom = get_tableu_bottom(&a, &b);
	let tableau_top: matrix<f64>;

	tableau_top = concatenate![
		Axis(1),
		if maximize { -(*c).to_owned() } else  { (*c).to_owned() },
		matrix::zeros((1, tableau_bottom.ncols() - c.ncols()))
	];
	
	 concatenate![Axis(0), tableau_top, tableau_bottom]
}

pub fn get_tableu_bottom(a: &matrix<f64>, b: &matrix<f64>) -> matrix<f64> {
	let bottom_left = concatenate![Axis(1), *a, matrix::eye(a.nrows())];
	concatenate![Axis(1), bottom_left, *b]
}
