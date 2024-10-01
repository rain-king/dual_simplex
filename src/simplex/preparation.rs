use core::f64;
use ndarray::{concatenate, Array2 as matrix, Axis};

<<<<<<< HEAD
pub fn original_tableau(maximize: bool, c: &matrix<f64>, a: &matrix<f64>, b: &&matrix<f64>) -> matrix<f64> {
	let tableau_bottom = get_tableu_bottom(a, b);
=======

pub fn original_tableau(maximize: bool, c: &matrix<f64>, a: &matrix<f64>, b: &&matrix<f64>) -> matrix<f64> {
	let tableau_bottom = get_tableu_bottom(a, b, false);
>>>>>>> dual_problem

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

<<<<<<< HEAD
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
=======
pub fn initialize(maximize: bool, c: &matrix<f64>, a: &matrix<f64>, b: &matrix<f64>) -> (matrix<f64>, bool) {
	let tableau_top: matrix<f64>;
	let tableau_bottom: matrix<f64>;
	let dual_problem = b.column(0).iter().all(|&x| x >= 0.0);
	if dual_problem {
		let c_transposed = c.t().to_owned();
		let b_transposed = b.t();
		tableau_bottom = get_tableu_bottom(&a, &c_transposed, true);
		tableau_top = concatenate![
			Axis(1),
			// has to start optimal in minimization or maximization
			if !maximize { b_transposed.to_owned() } else { -b_transposed.to_owned() }, 
			matrix::zeros((1, tableau_bottom.ncols() - b_transposed.ncols()))
		];
	} else {
		tableau_bottom = get_tableu_bottom(&a, &b, false);
		tableau_top = concatenate![
			Axis(1),
			if maximize { -(*c).to_owned() } else { (*c).to_owned() },
			matrix::zeros((1, tableau_bottom.ncols() - c.ncols()))
		];
	}

	 (concatenate![Axis(0), tableau_top, tableau_bottom], dual_problem)
}

pub fn get_tableu_bottom(a: &matrix<f64>, b: &matrix<f64>, transpose: bool) -> matrix<f64> {
	if transpose {
		let bottom_left = &concatenate![Axis(1), (*a).t(), -matrix::eye(a.ncols())];
		concatenate![Axis(1), -(*bottom_left).to_owned(), -(*b).to_owned()]
	} else {
		let bottom_left = &concatenate![Axis(1), *a, matrix::eye(a.nrows())];
		concatenate![Axis(1), *bottom_left, *b]
	}
>>>>>>> dual_problem
}
