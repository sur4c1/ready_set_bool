/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   ex07.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bguyot <bguyot@student.42mulhouse.fr>      +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/12/22 17:44:54 by bguyot            #+#    #+#             */
/*   Updated: 2023/12/27 09:57:27 by bguyot           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::exs::sat::sat;

pub fn ex07() {
	let formula = "AB&";
	println!("Formula {} is satisfiable: {}", formula, sat(formula));
	let formula = "AB|";
	println!("Formula {} is satisfiable: {}", formula, sat(formula));
	let formula = "AA^";
	println!("Formula {} is satisfiable: {}", formula, sat(formula));
	let formula = "AA!&";
	println!("Formula {} is satisfiable: {}", formula, sat(formula));
	let formula = "A!B&";
	println!("Formula {} is satisfiable: {}", formula, sat(formula));
	let formula = "ABCDEFGH&&&&&&&";
	println!("Formula {} is satisfiable: {}", formula, sat(formula));
}
