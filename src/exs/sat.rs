/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   sat.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bguyot <bguyot@student.42mulhouse.fr>      +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/12/22 17:41:24 by bguyot            #+#    #+#             */
/*   Updated: 2023/12/22 17:44:43 by bguyot           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::exs::eval_formula::eval_formula_opt;
use crate::exs::print_truth_table::find_variables;
use crate::exs::print_truth_table::get_test_form;

pub fn sat(formula: &str) -> bool {
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
		return false;
	}
	while test < 1 << variables.len() {
		let test_formula = get_test_form(formula, &variables, test);
		let res = eval_formula_opt(&test_formula);
		if res.is_none() {
			eprintln!(
				"print_truth_table: Formula {} lead to an error with test #{} ({})",
				formula, test, test_formula
			);
			return false;
		}
		if res.unwrap() {return true;}
		test += 1;
	}
	return false;
}
