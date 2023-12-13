/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   ex03.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bguyot <bguyot@student.42mulhouse.fr>      +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/12/05 16:15:38 by bguyot            #+#    #+#             */
/*   Updated: 2023/12/05 16:19:30 by bguyot           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::exs::eval_formula::eval_formula;

pub fn ex03() {
	let formula = "10&";
	println!("{} evaluate to {}", formula, eval_formula(formula));
	let formula = "10|";
	println!("{} evaluate to {}", formula, eval_formula(formula));
	let formula = "11>";
	println!("{} evaluate to {}", formula, eval_formula(formula));
	let formula = "10=";
	println!("{} evaluate to {}", formula, eval_formula(formula));
	let formula = "1011||=";
	println!("{} evaluate to {}", formula, eval_formula(formula));
}
