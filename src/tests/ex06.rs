/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   ex06.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bguyot <bguyot@student.42mulhouse.fr>      +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/12/21 17:58:04 by bguyot            #+#    #+#             */
/*   Updated: 2023/12/21 18:05:23 by bguyot           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::exs::conjunction_normal_form::conjunction_normal_form;

pub fn ex06()
{
	let formula = "AB&!";
	println!("The CNF form of {} is {}", formula, conjunction_normal_form(formula));
		let formula = "AB|!";
	println!("The CNF form of {} is {}", formula, conjunction_normal_form(formula));
	let formula = "AB|C&";
	println!("The CNF form of {} is {}", formula, conjunction_normal_form(formula));
	let formula = "AB|C|D|";
	println!("The CNF form of {} is {}", formula, conjunction_normal_form(formula));
	let formula = "AB&C&D&";
	println!("The CNF form of {} is {}", formula, conjunction_normal_form(formula));
	let formula = "AB&!C!|";
	println!("The CNF form of {} is {}", formula, conjunction_normal_form(formula));
	let formula = "AB|!C!&";
	println!("The CNF form of {} is {}", formula, conjunction_normal_form(formula));

}
