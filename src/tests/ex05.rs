/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   ex05.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bguyot <bguyot@student.42mulhouse.fr>      +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/12/21 11:30:36 by bguyot            #+#    #+#             */
/*   Updated: 2023/12/21 16:56:26 by bguyot           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::exs::negation_normal_form::negation_normal_form;

pub fn ex05() {
	let formula = "AB|!";
	println!(
		"The NNF of {} is {}",
		formula,
		negation_normal_form(formula)
	);
	let formula = "AB&!";
	println!(
		"The NNF of {} is {}",
		formula,
		negation_normal_form(formula)
	);
	let formula = "AB>";
	println!(
		"The NNF of {} is {}",
		formula,
		negation_normal_form(formula)
	);
	let formula = "AB^";
	println!(
		"The NNF of {} is {}",
		formula,
		negation_normal_form(formula)
	);
	let formula = "A";
	println!(
		"The NNF of {} is {}",
		formula,
		negation_normal_form(formula)
	);
	let formula = "A!!";
	println!(
		"The NNF of {} is {}",
		formula,
		negation_normal_form(formula)
	);
	let formula = "AB|C&!";
	println!(
		"The NNF of {} is {}",
		formula,
		negation_normal_form(formula)
	);
}
