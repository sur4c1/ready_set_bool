/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   print_truth_table.rs                               :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bguyot <bguyot@student.42mulhouse.fr>      +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/12/05 16:33:52 by bguyot            #+#    #+#             */
/*   Updated: 2023/12/13 11:18:20 by bguyot           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::exs::eval_formula::eval_formula_opt;
use std::collections::HashSet;
use std::vec::Vec;

struct Variable {
	name: char,
	mask: u32,
}

pub fn print_truth_table(formula: &str) {
	let variables = find_variables(formula);
	let mut test = 0;

	if variables.len() == 0 {
		let res = eval_formula_opt(formula);
		if res.is_some() {
			println!(
				"The formula {} is constant and evaluate to {}.",
				formula,
				res.unwrap()
			);
		}
		return;
	}
	// Draw first line
	print!("|");
	for v in &variables {
		print!(" {} |", v.name);
	}
	println!(" = |");
	// Draw separator line
	print!("|");
	for _v in &variables {
		print!("---|");
	}
	println!("---|");
	// Draw each line
	while test < 1 << variables.len() {
		let test_formula = get_test_form(formula, &variables, test);
		let res = eval_formula_opt(&test_formula);
		if res.is_none() {
			eprintln!(
				"print_truth_table: Formula {} lead to an error with test #{} ({})",
				formula, test, test_formula
			);
			return;
		}

		let res_char = if res.unwrap() == true { "1" } else { "0" };

		print!("|");
		for v in &variables {
			print!(" {} |", if (v.mask & test) == 0 { "0" } else { "1" });
		}
		println!(" {} |", res_char);
		test += 1;
	}
}

fn find_variables(formula: &str) -> Vec<Variable> {
	let mut var_names: HashSet<char> = HashSet::new();
	let mut ret: Vec<Variable> = Vec::new();
	let mut mask;

	for c in formula.chars() {
		if c.is_ascii_uppercase() {
			var_names.insert(c);
		}
	}

	if var_names.len() == 0 {
		return ret;
	}

	mask = 1 << (var_names.len() - 1);
	for c in "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
		if var_names.contains(&c) {
			let elem = Variable {
				name: c,
				mask: mask,
			};
			ret.push(elem);
			mask >>= 1;
		}
	}

	return ret;
}

fn get_test_form<'a>(formula: &'a str, variables: &Vec<Variable>, test: u32) -> String {
	let mut tmp = formula.to_string();

	for v in variables {
		tmp = tmp.replace(
			v.name.to_string().as_str(),
			if (v.mask & test) == 0 { "0" } else { "1" },
		);
	}

	return tmp;
}
