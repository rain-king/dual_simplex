// #![allow(warnings)]
mod ndarray_io;
mod simplex;
use ndarray_io as io;
use simplex::dual_simplex;
use std::io::stdin;
use std::io::BufRead; // needed to read empty lines without storing them

fn main() {
	println!("This is a program that solves the maximization or minimization of");
	println!("Z = cx subject to Ax <= b, A_eq = b_eq, with free b and b_eq,");
	println!("but x >= 0.");
	println!();

	println!("{}\n{}", "Write 1 if this is a maximization problem,",
		"or 0 if it's a minimization problem, then press Return again.");
	let maximize = read_bool();
	stdin().lock().lines().next(); // read empty line

	let c = io::read_row("Enter the c vector values separated by spaces, then press return again.");
	stdin().lock().lines().next(); // read empty line

	let a = io::read_matrix(&format!("{}{}",
		 "Enter the A matrix values row by row, with values separated by spaces,\n",
		 "then press Return again."));

	let b = io::read_column("Enter the b column values separated by spaces, then press return again.");
	stdin().lock().lines().next();
	
	dual_simplex(maximize, &c, &a, &b);
}

fn read_bool() -> bool {
	let mut input = String::new();
	stdin().read_line(&mut input).unwrap();
	input.trim() == "1"
}
