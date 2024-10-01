use core::f64;
use crate::ndarray_io::pretty_print_array2;
use ndarray::{s, Array1 as vector, Array2 as matrix};
mod preparation;
use preparation::{original_tableau, initialize};

pub fn dual_simplex(maximize: bool, c: &matrix<f64>, a: &matrix<f64>, b: &matrix<f64>) -> Vec<(usize, usize)> {
	let mut tableau: matrix<f64>;
	let mut basis: Vec<(usize, usize)> = Vec::new();

	println!();
	tableau = original_tableau(maximize, &c, &a, &b);
	println!("The initial tableau is:");
	pretty_print_array2(&tableau);
	println!();

<<<<<<< HEAD
	tableau = initialize(maximize, &c, &a, &b);
	
	basis = iterations(maximize, &mut tableau);
=======
	let dual_problem: bool;
	(tableau, dual_problem) = initialize(maximize, &c, &a, &b);
	
	dbg!(dual_problem);
	
	basis = iterations(&mut tableau);
>>>>>>> dual_problem
	println!("The final tableau is:");
	pretty_print_array2(&tableau);
	println!();

	basis
}

<<<<<<< HEAD
fn iterations(maximize: bool, tableau: &mut matrix<f64>) -> Vec<(usize, usize)> {
=======
fn iterations(tableau: &mut matrix<f64>) -> Vec<(usize, usize)> {
>>>>>>> dual_problem
	let mut basis = initialize_basis(tableau.to_owned());

	let mut iteration = 0;
	while not_feasible(tableau) {
		println!("Iteration {iteration}");
		pretty_print_array2(&tableau);
		println!();
		
<<<<<<< HEAD
		if iteration == 5 {
			panic!();
		}
		
		let (pivot_row_index, pivot_column_index) = pivot(tableau, maximize, &basis);
=======
		let (pivot_row_index, pivot_column_index) = pivot(tableau, &basis);
>>>>>>> dual_problem
		for element in basis.iter_mut() {
			// variable with pivot row enters, variable with pivot column exits
			if element.0 == pivot_row_index {
				*element = (pivot_row_index, pivot_column_index);
			}
		}
<<<<<<< HEAD

=======
		
		if iteration == 5 {
			break;
		}
		
>>>>>>> dual_problem
		iteration += 1;
	}

	basis
}

fn initialize_basis(tableau: matrix<f64>) -> Vec<(usize, usize)> {
	let mut basis = Vec::new();
	for j in 0..(tableau.ncols() - 1) {
		// avoid right hand side
		let col = tableau.column(j).slice(s![1..]).to_owned();
		if is_basic(col) {
			for i in 1..tableau.nrows() {
				if tableau[(i, j)] == 1.0 {
					basis.push((i, j));
				}
			}
		}
	}

	basis
}

fn is_basic(column: vector<f64>) -> bool {
	let has_only_one_1 = column.iter().filter(|&&x| x == 1.0).count() == 1;
	let everything_else_is_0 = column.iter().filter(|&&x| x == 0.0).count() == column.len() - 1;
	has_only_one_1 && everything_else_is_0
}

<<<<<<< HEAD
fn not_feasible(tableau: &mut matrix<f64>) -> bool {
	tableau.column(tableau.ncols() - 1).slice(s![1..]).into_iter().any(|&x| x < 0.0)
}

fn pivot(tableau: &mut matrix<f64>, maximize: bool, basis: &Vec<(usize, usize)>) -> (usize, usize) {
	let (pivot_row_index, pivot_column_index) = pivot_indexes(tableau, maximize, basis);
=======
fn not_feasible(tableau: &matrix<f64>) -> bool {
	tableau.column(tableau.ncols() - 1).slice(s![1..]).iter().any(|&x| x < 0.0)
}

fn pivot(tableau: &mut matrix<f64>, basis: &Vec<(usize, usize)>) -> (usize, usize) {
	let (pivot_row_index, pivot_column_index) = pivot_indexes(tableau, basis);
>>>>>>> dual_problem

	let normalization_scalar = tableau[(pivot_row_index, pivot_column_index)].to_owned();
	
	tableau.row_mut(pivot_row_index).map_inplace(|x| *x /= normalization_scalar);

	let pivot_row = tableau.row(pivot_row_index).to_owned();
	for mut row in tableau.rows_mut().into_iter() {
		if row != pivot_row {
			let pivot_value = row[pivot_column_index];
			row.zip_mut_with(&pivot_row, |r, p| *r -= p * pivot_value);
		}
	}

	(pivot_row_index, pivot_column_index)
}

<<<<<<< HEAD
fn pivot_indexes(tableau: &mut matrix<f64>, maximize: bool, basis: &Vec<(usize, usize)>) -> (usize, usize) {
	let pivot_row_index = argmin(&tableau.column(tableau.ncols() - 1).slice(s![1..]).to_owned()) + 1;
	let mut basis_cols = basis.iter().map(|x| x.1).into_iter();

	let mut pivot_column_index = 0 as usize;
	let mut optimal_quotient = if maximize { f64::INFINITY } else { f64::NEG_INFINITY };
	for (j, pivot_value) in tableau.row(pivot_row_index).into_iter().enumerate() {
		if j < tableau.ncols() - 1 && !(basis_cols.any(|c| c == j)) {
			let z_j_minus_c_j = tableau[(0, j)];
			if *pivot_value < 0.0 {
				let quotient_condition = if maximize {
					(z_j_minus_c_j / pivot_value).abs() < optimal_quotient 
				} else {
					(z_j_minus_c_j / pivot_value).abs() > optimal_quotient
				};
				if quotient_condition {
					optimal_quotient = z_j_minus_c_j / pivot_value;
=======
fn pivot_indexes(tableau: &mut matrix<f64>, basis: &Vec<(usize, usize)>) -> (usize, usize) {
	let pivot_row_index = argmin(&tableau.column(tableau.ncols() -1).slice(s![1..]).to_owned()) + 1;
	
	if tableau.row(pivot_row_index).slice(s![0..-1]).iter().all(|&x| x >= 0.0) {
		panic!("!!! The problem has no feasible solution.")
	}
	
	let mut basis_cols = basis.iter().map(|x| x.1).into_iter();

	let mut pivot_column_index = 0;
	let mut optimal_quotient = if true {f64::INFINITY} else {f64::NEG_INFINITY};
	for (j, &pivot_value) in tableau.row(pivot_row_index).into_iter().enumerate() {
		if j < tableau.ncols() - 1 && !(basis_cols.any(|c| c == j)) {
			let z_j_minus_c_j = tableau[(0, j)];
			let pivot_column_condition = pivot_value < 0.0;
			
			if pivot_column_condition {
				let current_quotient_abs = (z_j_minus_c_j / pivot_value).abs();
				if current_quotient_abs < optimal_quotient {
					optimal_quotient = current_quotient_abs;
>>>>>>> dual_problem
					pivot_column_index = j;
				}
			}
		}
	}

	(pivot_row_index, pivot_column_index)
}

fn argmin(arr: &vector<f64>) -> usize {
	let mut min = f64::INFINITY;
	let mut argmin: usize = 0;

<<<<<<< HEAD
	for (i, value) in arr.into_iter().enumerate() {
		if *value < min {
			min = *value;
=======
	for (i, &value) in arr.into_iter().enumerate() {
		if value < min {
			min = value;
>>>>>>> dual_problem
			argmin = i;
		}
	}
	argmin
}

<<<<<<< HEAD
fn argmax(arr: &vector<f64>) -> usize {
	let mut max = -f64::INFINITY;
	let mut argmax: usize = 0;

	for (i, value) in arr.into_iter().enumerate() {
		if *value > max {
			max = *value;
			argmax = i;
		}
	}
	argmax
}
=======
// fn argmax(arr: &vector<f64>) -> usize {
// 	let mut max = -f64::INFINITY;
// 	let mut argmax: usize = 0;

// 	for (i, &value) in arr.into_iter().enumerate() {
// 		if value > max {
// 			max = value;
// 			argmax = i;
// 		}
// 	}
// 	argmax
// }
>>>>>>> dual_problem
